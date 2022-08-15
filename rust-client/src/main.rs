use std::{error::Error};
use imu::imu;

fn main() -> Result<(), Box<dyn Error>> {
    
    imu::test();

    Ok(())
}
