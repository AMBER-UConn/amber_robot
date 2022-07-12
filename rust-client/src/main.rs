use std::error::Error;

use rustodrive::{
    canproxy::CANProxy,
    odrivegroup::{ODriveGroup},
    commands::ODriveAxisState::*, threads::ReadWriteCANThread
};
use signal_hook::{consts::SIGINT, iterator::Signals};

fn main() -> Result<(), Box<dyn Error>> {
    let mut can_proxy = CANProxy::new("can1");

    can_proxy.register_rw("thread1", odrive_main);

    let stop_all = can_proxy.begin();

    let mut signals = Signals::new(&[SIGINT])?;
    for sig in signals.forever() {
        println!("\nQuitting the program {:?}", sig);
        break;
    }

    println!("all done!");
    Ok(())
}

fn odrive_main(can_read_write: ReadWriteCANThread) {
    let odrives = ODriveGroup::new(can_read_write, &[1, 2, 3, 4]);

    println!("Starting calibration sequence");
    odrives.all_axes(|ax| ax.set_state(FullCalibrationSequence));
    println!("Finished calibration sequence");

    let speeds_iter = [10, 10, 10, 10].iter();
    odrives.all_axes(|ax| speeds_iter.next());

    println!("Motors fully calibrated!")
}