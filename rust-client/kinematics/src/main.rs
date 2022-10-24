use std::f32::consts::PI;
use kinematics::bezier::bez_curve;
use nalgebra as na;
use na::Vector2;

mod plot;
mod forward_kinematics;
mod inverse_kinematics;
mod path_gen;
mod bezier;

fn main() {
    forward_kinematics::forward_ik::<f32>(PI/5.0, PI/5.0);

    // let theta = inverse_kinematics::inverse_ik::<f32>(1.414, 0.899);
    // let coordinates = forward_kinematics::forward_ik::<f32>(theta.x, theta.y);

    // let coordinates = forward_kinematics::forward_ik::<f32>(PI/5.0, PI/5.0);
    // let degrees = inverse_kinematics::inverse_ik::<f32>(coordinates.x, coordinates.y);

    // println!("Entered Angles are {} and {} degrees for which the coordinates are {}", PI/5.0, PI/5.0, coordinates);
    // println!("For coordinates the rotations are {}", degrees);
    
    let coord: (f32, f32) = bezier::bez_curve(0.4);
    // println!("bezier curve value {:?}", coord);

    type Vector2 = na::Vector2<f32>;
    let mut rotations: Vector2;
    for x in (0..100).step_by(1) {
        rotations = bezier::bez_curve_ik(x as f32 / 100.0); // calls selected bezier curve, returns a Vector
        // send rotations.x to hip joint
        // send rotations.y to knee joint
        println!("{:?}, {}", rotations, x as f32 / 100.0);
    }

    let theta = inverse_kinematics::inverse_ik::<f32>(coord.0, coord.1);
    // println!("theta is {}", theta);
    
    // println!("Coordinates: {}", coordinates);
    // let range = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    // let coefs = vec![(-170.0, -470.0), (-240.0, -470.0), (-300.0, -360.0), (-300.0, -360.0), 
    //                                         (-300.0, -360.0), (0.0, -360.0), (0.0, -360.0), (0.0, -320.0), 
    //                                         (300.0, -320.0), (300.0, -320.0), (240.0, -470.0), (170.0, -470.0)];
    // for i in range {
    //     println!("{:?}", path_gen::decasteljau(i, &coefs))
    // }

    plot::curve_plot();
}