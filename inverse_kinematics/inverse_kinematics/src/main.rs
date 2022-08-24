use na::Vector2;
use nalgebra as na;
use std::f32::consts::PI;

mod plot;

fn main() {
    forward_ik(0.0, 0.0);
    pseudo_inverse(0.0, 0.0);
    plot::graph();
}

fn urdf_parser() {
    unimplemented!("For lenghts of links OR displacements between joints");
}

fn forward_ik(theta1: f32, theta2: f32) {
    let c1: f32 = theta1.cos();
    let c2: f32 = theta2.cos();
    let s1: f32 = theta1.sin();
    let s2: f32 = theta2.sin();

    let c12: f32 = (theta1+theta2).cos();
    let s12: f32 = (theta1+theta2).sin();
    
    let l1:f32 = 1.0;
    let l2:f32 = 1.0;


    let x = l1*c1 + l2*c12;
    let y = l1*s1 + l2*s12;
    
    let fwd =  Vector2::new(x, y);
}

fn dh_forward_ik() {
    // implement the denavit hartenberg parameters
    // multiply the homogenous transformation matrices
    // get forward kinematics values
    todo!()
}

fn pseudo_inverse(theta1: f32, theta2: f32) {
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
    println!("pinv {}", pseudo_inverse_jacobian);

}   