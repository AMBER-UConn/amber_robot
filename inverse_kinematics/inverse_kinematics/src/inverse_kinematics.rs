use na::Vector2;
use nalgebra as na;


// Analytical Inverse Kinematic Solution
pub fn inverse_ik(x:f32, y:f32) {
    let l1:f32 = 1.0;
    let l2:f32 = 1.0;
    let theta2: f32 = ((x.powi(2) + y.powi(2) - l1.powi(2) - l2.powi(2))/2.0*l1*l2).acos();
    let theta1:f32 = (y/x).atan() - (l2*(theta2.sin())/(l1 + l2*(theta2.cos())));

    let inv =  Vector2::new(theta1, theta2);

    println!("Inverse IK : {}", inv);

}

pub fn pseudo_inverse(theta1: f32, theta2: f32) {
    let c1: f32 = theta1.cos();
    let c2: f32 = theta2.cos();
    let s1: f32 = theta1.sin();
    let s2: f32 = theta2.sin();

    let c12: f32 = (theta1+theta2).cos();
    let s12: f32 = (theta1+theta2).sin();
    
    let l1:f32 = 1.0;
    let l2:f32 = 1.0;

    let j0 = -l1*s1 -l2*s12;
    let j1 = -l2*s2;
    let j2 = l1*c1 + l2*c12;
    let j3 = l2*c12;

    let jacobian = na::Matrix2::new(j0, j1, j2, j3);
    let pseudo_inverse_jacobian = jacobian.singular_values();
    // println!("pinv {}", pseudo_inverse_jacobian);
}