use nalgebra as na;
use na::Vector2;

use crate::inverse_kinematics;
use crate::forward_kinematics;


/// Closed Form Solution or Analytical Solution
/// Uses Inverse Trigonometry
/// 
pub fn inverse_ik<T>(x:f32, y:f32) -> Vector2<f32>
{   
    type Vector2 = na::Vector2<f32>;
    println!("inputs = {} {}", x, y);
    let l1:f32 = 1.0;
    let l2:f32 = 1.0;
    let theta2_sol1: f32 = ((x.powi(2) + y.powi(2) - l1.powi(2) - l2.powi(2))/2.0*l1*l2).acos();
    let theta1_sol1:f32 = (y/x).atan() - (l2*(theta2_sol1.sin())/(l1 + l2*(theta2_sol1.cos()))).atan();
    

    // let theta2_sol2: f32 = -1.0 * (((x.powi(2) + y.powi(2) - l1.powi(2) - l2.powi(2))/2.0*l1*l2).acos());
    // let theta1_sol2:f32 = (y/x).atan() + (l2*(theta2_sol2.sin())/(l1 + l2*(theta2_sol2.cos()))).atan();

    let inv_sol1: Vector2 = Vector2::new(theta1_sol1, theta2_sol1);
    // let inv_sol2=  na::Vector2::new(theta1_sol2, theta2_sol2);

    // println!("Inverse IK : {}", inv_sol1);
    // println!("Inverse IK : {}", inv_sol2);
    return inv_sol1
}

pub fn _pseudo_inverse(theta1: f32, theta2: f32) {
    let c1: f32 = theta1.cos();
    let _c2: f32 = theta2.cos();
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
    let _pseudo_inverse_jacobian = jacobian.singular_values();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use rand::Rng;

    #[test]
    fn test_inverse_kinematics() {
        let mut rng = rand::thread_rng();
        let n: i16 = 5000;
        // assert_eq!(add(1, 2), 3);

        let x = 2.0*(rng.gen::<f32>());
        let y = 2.0*(rng.gen::<f32>());
        let theta = inverse_kinematics::inverse_ik::<f32>(x, y);
        let coordinates = forward_kinematics::forward_ik::<f32>(theta.x, theta.y);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        // assert_eq!(bad_add(1, 2), 3);
    }
}