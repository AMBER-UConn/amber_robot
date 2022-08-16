use k::prelude::*;

fn main() {
    // Load urdf file
    let chain = k::Chain::<f32>::from_urdf_file("../urdf/2dof_leg_module_v3.urdf").unwrap();
    println!("chain: {chain}");

    // Set initial joint angles
    let angles = vec![0.2, 0.2, 0.0, -1.0, 0.0, 0.0, 0.2, 0.2, 0.0, -1.0, 0.0, 0.0];

    chain.set_joint_positions(&angles).unwrap();
    println!("initial angles={:?}", chain.joint_positions());

    let target_link = chain.find("l_wrist_pitch").unwrap();

    // Get the transform of the end of the manipulator (forward kinematics)
    chain.update_transforms();
    let mut target = target_link.world_transform().unwrap();

    println!("initial target pos = {}", target.translation);
    println!("move z: +0.1");
    target.translation.vector.z += 0.1;

    // Create IK solver with default settings
    let solver = k::JacobianIkSolver::default();

    // Create a set of joints from end joint
    let arm = k::SerialChain::from_end(target_link);
    // solve and move the manipulator angles
    solver.solve(&arm, &target).unwrap();
    println!("solved angles={:?}", chain.joint_positions());

    chain.update_transforms();
    let solved_pose = target_link.world_transform().unwrap();
    println!("solved target pos = {}", solved_pose.translation);
}