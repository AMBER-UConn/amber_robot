use nalgebra as na;


pub fn forward_ik<T>(theta1: f32, theta2: f32) -> na::Vector2<f32> {
    
    type Vector2 = na::Vector2<f32>;
    
    let c1: f32 = theta1.cos();
    let _c2: f32 = theta2.cos();
    let s1: f32 = theta1.sin();
    let _s2: f32 = theta2.sin();

    let c12: f32 = (theta1+theta2).cos();
    let s12: f32 = (theta1+theta2).sin();
    
    let l1:f32 = 1.0;
    let l2:f32 = 1.0;


    let x = l1*c1 + l2*c12;
    let y = l1*s1 + l2*s12;
    
    let fwd: Vector2 =  Vector2::new(x, y);
    // println!("Forward : {} {}", fwd.x, fwd.y);
    // println!("Forward IK : {}", fwd);

    return fwd
}

pub fn _dh_forward_ik() {
    // implement the denavit hartenberg parameters
    // multiply the homogenous transformation matrices
    // get forward kinematics values
    todo!()
}