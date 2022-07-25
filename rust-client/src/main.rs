use std::error::Error;

use rustodrive::{commands::{ODriveAxisState::*, Write}, canproxy::CANProxy, threads::ReadWriteCANThread, odrivegroup::ODriveGroup};
use signal_hook::{consts::SIGINT, iterator::Signals};


fn odrive_main(can_read_write: ReadWriteCANThread) {
    let odrives = ODriveGroup::new(can_read_write, &[0, 1]);

    odrives.all_axes(|ax| ax.set_state(ClosedLoop));
    odrives.all_axes(|ax| ax.motor.set_input_vel(10.0));
    
    //InputVel when [1; 8]: [1.40e-45, 3.59e-43, 9.18e-41, 2.35e-38, 0, 0, 0, 0]
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut can_proxy = CANProxy::new("can0");
    can_proxy.register_rw("thread 1", odrive_main);
    let stop_all = can_proxy.begin();

    let mut signals = Signals::new(&[SIGINT])?;
    for sig in signals.forever() {
        println!("\nQuitting the program {:?}", sig);
        break;
    }
    stop_all().unwrap();
    println!("all done!");
    Ok(())
}