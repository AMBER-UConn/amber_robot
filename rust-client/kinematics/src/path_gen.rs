use flo_curves::*;
use flo_curves::bezier::*;

// mod inverse_kinematics;

struct CurvePoints {
    p0: Coord2,
    cp: Vec<Coord2>,
    p1: Coord2,
}

// pub fn generate_curve(p: Vec<Coord2>) -> Curve<Coord2> {
//     let curve = Curve::from_points(p[0], p[1], p[2]);//(Coord2(100.0, 500.0), (Coord2(400.0, 2000.0), Coord2(1500.5, 1200.0)), Coord2(1400.0, 900.0));
//     curve
// }

pub fn walk_curve() -> Curve<Coord2>    {
    //let p = vec![Coord2(100.0, 500.0), (Coord2(400.0, 2000.0), Coord2(1500.5, 1200.0)), Coord2(1400.0, 900.0)];
    let curve = Curve::from_points(Coord2(100.0, 500.0), (Coord2(400.0, 2000.0), Coord2(1500.5, 1200.0)), Coord2(1400.0, 900.0));
    for section in walk_curve_evenly(&curve, curve_length(&curve, 0.1)/1000.0, 0.1) {
        let (_t_min, t_max) = section.original_curve_t_values();
        let pos = curve.point_at_pos(t_max);
        // println!("{:?}", pos.0);
        // let a = pos.0;
        // println!("{}", a);

        // put what to do with point here
        //let sol = inverse_ik(t_max, pos);
    }
    return curve
}

pub fn decasteljau(t: f32, coefs: &Vec<(f32, f32)>) -> (f32, f32) {
    let mut points = coefs.clone();
    let n = points.len();
    for j in 1..n {
        for k in 0..(n-j) {
            points[k].0 = points[k].0*(1.0-t) + points[k+1].0*t;
            points[k].1 = points[k].1*(1.0-t) + points[k+1].1*t;
        }
    }
    return points[0];
}

/*
* S = step length
* H = step height
* Tm = period of flight phase and stand phase
* Te = period of swing-back movement and retraction movement of flight phase
* Tn = main motion period of removal of swing-back and retraction of flight phase
*
* Tm = Tn + 2Te
* Te and Tn ARE NOT USED IN THIS VERSION
*/
pub fn new_eq(S: f32, H: f32, Tm: f32, t: f32) -> (f32, f32) {
    let mut x = 0.0;
    let mut y = 0.0;
    let val: f32 = 6.28*(t/Tm);
    
    x = (S * ((t/Tm)-((1.0/(6.28))*val.sin()))) - (S/2.0);

    if t < (Tm/2.0) {
        y = (2.0*H) * ((t/Tm) - ((1.0/12.56)*(2.0*val).sin()));
    } else {
        y = (2.0*H) * (1.0 - (t/Tm) - ((1.0/12.56)*(2.0*val).sin()));
    }

    println!("{},{}",x,y);
    return (x,y)
}

pub fn new_eq_2(S: f32, H: f32, Tm: f32, Te: f32, Tn: f32, t: f32) -> (f32, f32) {
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let val: f32 = 6.28*((t-Te)/Tn);
    let C2: f32 = 400.0;
    let C3: f32 = 4.0;
    let start_val = ((S*Te)/(6.28*Tm)) * (3.14 as f32).sin();

    if t <= Te {
        x = -1.0*start_val - (S/(2.0*Te)) + C2;
    } else if t <= (Te + Tn) {
        x = (S * (((t-Te)/Tn)-((1.0/(6.28))*val.sin()))) - (S/2.0);
    } else {
        x = start_val*((t-Te-Tn)/Te) - ((S/(2.0*Tm))*(t-Te-Tn)) + C3;
    }

    if t < (Tm/2.0) {
        y = (2.0*H) * ((t/Tm) - ((1.0/12.56)*(2.0*val).sin()));
    } else {
        y = (2.0*H) * (1.0 - (t/Tm) - ((1.0/12.56)*(2.0*val).sin()));
    }

    println!("{},{}",x,y);
    return (x,y)
}