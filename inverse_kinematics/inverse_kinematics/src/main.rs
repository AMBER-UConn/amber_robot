use std::f32::consts::PI;

mod plot;
mod urdf_parser;
mod forward_kinematics;
mod inverse_kinematics;

fn main() {
    forward_kinematics::forward_ik(PI/2.0, 0.0);
    inverse_kinematics::pseudo_inverse(0.0, 0.0);
    plot::graph();
    urdf_parser::parse();
    inverse_kinematics::inverse_ik(0.0, 2.0);
}