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

pub fn walk_curve() {
    // let p = vec![Coord2(100.0, 500.0), (Coord2(400.0, 2000.0), Coord2(1500.5, 1200.0)), Coord2(1400.0, 900.0)];
    let curve = Curve::from_points(Coord2(100.0, 500.0), (Coord2(400.0, 2000.0), Coord2(1500.5, 1200.0)), Coord2(1400.0, 900.0));
    for section in walk_curve_evenly(&curve, curve_length(&curve, 0.1)/20.0, 0.1) {
        let (_t_min, t_max) = section.original_curve_t_values();
        let pos = curve.point_at_pos(t_max);
        println!("{:?}", pos);
        // put what to do with point here
        //let sol = inverse_ik(t_max, pos);
    }
}