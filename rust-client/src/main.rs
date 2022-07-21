use std::error::Error;

use rustodrive::{commands::{ODriveAxisState::*, Write}, canproxy::CANProxy, threads::ReadWriteCANThread, odrivegroup::ODriveGroup};
use signal_hook::{consts::SIGINT, iterator::Signals};


fn odrive_main(can_read_write: ReadWriteCANThread) {
    let odrives = ODriveGroup::new(can_read_write, &[1, 2, 3, 4]);

    odrives.all_axes(|ax| ax.set_state(ClosedLoop));
    odrives.all_axes(|ax| ax.send_command(Write::SetInputVelocity, 255));
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