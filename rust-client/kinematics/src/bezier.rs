use nalgebra as na;
use na::Vector2;
use crate::inverse_kinematics;


pub struct Bezier {
    p0: na::Vector2<f32>,
    p1: na::Vector2<f32>,
    p2: na::Vector2<f32>,
    p3: na::Vector2<f32>,
}

// implments cubic bezier curve
impl Bezier {
    //generates cubic bezier curve polynomial
    fn gen_curve(&self, t:f32) -> na::Vector2<f32> {
        let s0 = f32::powi(1.0-t, 3)*&self.p0;
        let s1 = 3.0*t*f32::powi(1.0-t, 2)*&self.p1;
        let s2 = 3.0*(1.0-t)*f32::powi(t, 2)*&self.p2;
        let s3 = f32::powi(t, 3)*&self.p3;

        let s = s0+s1+s2+s3;
        return s
    }
}

// for a parameter t from 0 to 1
// returns the value of the cubic bezier curve at t for the control points:
// (-5.4, -4.63), (-5.28, 5.38), (4.98, -1.4), (5.6, 5.67)
// (-13.716, -11.7602), (-13.4112, 13.6652), (12.6492, -3.556), (14.224, 14.3764)
pub fn bez_curve(t: f32) -> (f32, f32) {
    let bezier1 = Bezier {p0: na::Vector2::new(-13.716, -11.7602), p1: na::Vector2::new(-13.4112, 13.6652), p2: na::Vector2::new(12.6492, -3.556), p3: na::Vector2::new(14.224, 14.3764) };
    let x: f32 = bezier1.gen_curve(t).x;
    let y: f32 = bezier1.gen_curve(t).y;

    return (x, y)
}

// for a parameter t returns inverse kinematics rotations for 
// value of the cubic bezier curve at t
pub fn bez_curve_ik(t: f32) -> Vector2<f32>{

    let coord = bez_curve(t);
    type Vector2 = na::Vector2<f32>;
    let ik_sol: Vector2 = inverse_kinematics::inverse_ik::<f32>(coord.0, coord.1);
    return ik_sol
}