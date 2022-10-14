use k::prelude::*;
use std::f32::consts::PI;

pub fn ik() {
    // Load urdf file
    let chain = k::Chain::<f32>::from_urdf_file("/home/vihaan/Projects/amber/amber_robot/ik_module/urdf/2dof_leg_module_v3.urdf").unwrap();
    println!("chain: {chain}");

    // Set initial joint angles
    let angles = vec![0.0, 0.0];

    chain.set_joint_positions(&angles).unwrap();
    println!("initial angles={:?}", chain.joint_positions());

    let target_link = chain.find("knee_joint").unwrap();

    // Get the transform of the end of the manipulator (forward kinematics)
    chain.update_transforms();
    let mut target = target_link.world_transform().unwrap();

    println!("Target Link - {}, Target - {}", target_link, target);
    println!("initial target pos = {}", target.translation);
    println!("move z: +0.1");
//     println!("Translation = {}", target.translation);
//     target.translation.vector.z += 1.0;
//     chain.update_transforms();
//     let mut target = target_link.world_transform().unwrap();


//     println!("Translation = {}", target.translation);


//     // Create IK solver with default settings
//     let solver = k::JacobianIkSolver::default();

//     // Create a set of joints from end joint
//     let arm = k::SerialChain::from_end(target_link);
//     // solve and move the manipulator angles

//     let constraints = k::Constraints {
//         rotation_x: false,
//         rotation_z: false,
//         // rotation_y: false,
//         position_x: false,
//         position_y: false,
//         position_z: false,
//         ..Default::default()
//     };

//     solver
//    .solve_with_constraints(&arm, &target, &constraints)
//    .unwrap_or_else(|err| {
//        println!("Err: {err}");
//    });    println!("solved angles={:?}", chain.joint_positions());

//     chain.update_transforms();
//     let solved_pose = target_link.world_transform().unwrap();
//     println!("solved target pos = {} and solved angles ={:?}", solved_pose.translation, chain.joint_positions());
   }