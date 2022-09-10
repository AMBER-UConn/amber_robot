use std::f32::consts::PI;
use path_gen::walk_curve;

mod plot;
mod urdf_parser;
mod forward_kinematics;
mod inverse_kinematics;
mod path_gen;
mod bezier;

fn main() {
    // forward_kinematics::forward_ik::<f32>(0.0, PI/2.0);
    // // inverse_kinematics::pseudo_inverse(0.0, 0.0);
    // plot::graph();
    // // urdf_parser::parse();
    // // inverse_kinematics::inverse_ik(1.0, 1.0);
    // // let x:f32 = 0.0;
    // // println!("print this {}", x.acos());

    // println!("Limits for x is -2.0 to 2.0");
    // println!("Limits for y is -2.0 to 2.0");

    // let theta = inverse_kinematics::inverse_ik::<f32>(1.414, 0.899);
    // let coordinates = forward_kinematics::forward_ik::<f32>(theta.x, theta.y);
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