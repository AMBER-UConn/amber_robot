use rustodrive::{
    canproxy::CANProxy,
    odrivegroup::ODriveGroup,
    threads::ReadWriteCANThread, 
    casts::{Heartbeat, Temperature}, 
    response::{Success}, 
    utils::ResultAll,
    state::{AxisState::*, ReadComm::*, WriteComm::*, ControlMode::*, InputMode::*},
};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error};
use kinematics::{
    inverse_kinematics::inverse_ik_rots
};

pub mod test_ui;

fn init_motors(odrv: &ODriveGroup) {
    //odrv.all_axes(|ax| ax.set_state(EncoderIndexSearch));
    std::thread::sleep_ms(2000);
}

// TODO update documentation to reflect new changes
fn odrive_main(can_read_write: ReadWriteCANThread) {
    let odrives = ODriveGroup::new(can_read_write, &[0, 1, 2, 3, 4, 5]);

    init_motors(&odrives);

    odrives.all_axes::<(), _>(|ax| ax.set_state(Idle));
    let heartbeat: Vec<Success<Heartbeat>> = odrives.all_axes(|ax| ax.get_heartbeat()).unwrap_all();
    //let heartbeat: Success<Heartbeat> = odrives.axis(&1, |ax| ax.get_heartbeat()).unwrap();

    let temp: Vec<Success<Temperature>> = odrives.all_axes(|ax| ax.get_temperatures()).unwrap_all();

    println!("hb: {:?}\n temp: {:?}", heartbeat, temp);

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
