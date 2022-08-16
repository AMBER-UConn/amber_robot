use k::*;

fn main() {
    // // Load urdf file
    // let chain = k::Chain::<f32>::from_urdf_file("/home/vihaan/Projects/amber/amber_robot/ik_module/urdf/2dof_leg_module_v3.urdf").unwrap();
    // println!("chain: {chain}");
    // assert_eq!(chain.dof(), 2);

    // // // Set initial joint angles
    // // let angles = vec![0.2, 0.2, 0.0, -1.0, 0.0, 0.0, 0.2, 0.2, 0.0, -1.0, 0.0, 0.0];
    // let angles = vec![0.0, 0.0];
    // println!("angles: {:?}", angles);


    // chain.set_joint_positions(&angles).unwrap();
    // println!("initial angles={:?}", chain.joint_positions());

    // let target_link = chain.find("hip_joint").unwrap();

    // // Get the transform of the end of the manipulator (forward kinematics)
    // chain.update_transforms();
    // let mut target = target_link.world_transform().unwrap();

    // println!("initial target pos = {}", target.translation);
    // // println!("move z: +0.1");
    // // target.translation.vector.z += 0.1;

    // // Create IK solver with default settings
    // let solver = k::JacobianIkSolver::default();

    // // Create a set of joints from end joint
    // let arm = k::SerialChain::from_end(target_link);
    // println!("arm dof {}", arm.dof());

    // // solve and move the manipulator angles
    // solver.solve(&arm, &target).unwrap();
    // println!("solved angles={:?}", chain.joint_positions());

    // chain.update_transforms();
    // let solved_pose = target_link.world_transform().unwrap();
    // println!("solved target pos = {}", solved_pose.translation);




    use k::prelude::*;

    let chain = k::Chain::<f32>::from_urdf_file("/home/vihaan/Projects/amber/amber_robot/ik_module/urdf/2dof_leg_module_v3.urdf").unwrap();
    // Create sub-`Chain` to make it easy to use inverse kinematics
    let target_joint_name = "hip_joint";
    let r_wrist = chain.find(target_joint_name).unwrap();
    let mut arm = k::SerialChain::from_end(r_wrist);
    println!("arm: {arm}");

    // Set joint positions of `arm`
    let positions = vec![0.0];
    arm.set_joint_positions(&positions).unwrap();
    println!("initial positions={:?}", arm.joint_positions());

    // Get the transform of the end of the manipulator (forward kinematics)
    let mut target = arm.update_transforms().last().unwrap().clone();

    println!("initial target pos = {}", target.translation);
    println!("move x: -0.1");
    target.translation.vector.x -= 0.1;
    
    // Create IK solver with default settings
    let solver = k::JacobianIkSolver::default();

    let mut constraints = k::Constraints::default();
    constraints.rotation_x = false;
    constraints.rotation_z = false;
    constraints.position_x = false;
    constraints.position_y = false;
    constraints.position_z = false;

    solver
        .solve_with_constraints(&arm, &target, &constraints)
        .unwrap_or_else(|err| {
            println!("Err: {err}");
   });
    // solve and move the manipulator positions
    // solver.solve(&arm, &target).unwrap();


    println!("solved positions={:?}", arm.joint_positions());
}