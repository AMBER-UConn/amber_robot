use urdf_rs;

#[allow(dead_code)]
pub fn parse() {
    let urdf_robo = urdf_rs::read_file("./urdf/2dof_leg_module_v3.urdf").unwrap();
    let links = urdf_robo.links;
    // println!("Link 1 - {:?}", links[0].visual[0].origin);
    // println!("Link 2 - {:?}", links[1].visual[0].origin.xyz);
    // println!("Link 3 - {:?}", links[2].visual[0].origin.xyz);


    let joints = urdf_robo.joints;
    // println!("Joint - 1 {:?}", joints[0].origin.xyz);
    // println!("Joint - 2 {:?}", joints[1].origin.xyz);
}
