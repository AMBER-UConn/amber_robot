use rustodrive::{
    canproxy::CANProxy,
    state::{ODriveAxisState::*, ControlMode, InputMode},
    odrivegroup::ODriveGroup,
    threads::ReadWriteCANThread,
};
use signal_hook::{consts::SIGINT, iterator::Signals};
use ui_test::ui_main;
use std::{error::Error};

pub mod ui_test;

fn init_motors(odrv: &ODriveGroup) {
    odrv.all_axes(|ax| ax.set_state(EncoderIndexSearch));
}

fn odrive_main(can_read_write: ReadWriteCANThread) {
    let odrives = ODriveGroup::new(can_read_write, &[0, 1, 2, 3, 4, 5]);
}

fn main() -> Result<(), Box<dyn Error>> {
    // let mut can_proxy = CANProxy::new("can0");
    // can_proxy.register_rw("thread 1", ui_main);
    // let stop_all = can_proxy.begin();

    ui_main();

    let mut signals = Signals::new(&[SIGINT])?;
    for sig in signals.forever() {
        println!("\nQuitting the program {:?}", sig);
        break;
    }
    // stop_all().unwrap();
    println!("all done!");
    Ok(())
}
