use rustodrive::{
    canproxy::CANProxy,
    state::{AxisState::*, ControlMode, InputMode},
    odrivegroup::ODriveGroup,
    threads::ReadWriteCANThread,
    utils::*,
    response::{Success}
};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error};
pub mod controller;
use crate::controller::controller;


fn init_motors(odrv: &ODriveGroup) {
    let a: Vec<Success<()>> = odrv.all_axes(|ax| ax.set_state(EncoderIndexSearch)).unwrap_all();
}

fn odrive_main(can_read_write: ReadWriteCANThread) {
    let odrives = ODriveGroup::new(can_read_write, &[0, 1]);

    //init_motors(&odrives);

    // test_ui::ui_start(odrives);


    //odrives.all_axes(|ax| ax.set_state(ClosedLoop));

    controller(odrives);
    //let b: Vec<Success<()>> = odrives.all_axes(|ax| ax.motor.set_input_vel(0.0)).unwrap_all();

    //let b: Vec<Success<()>>  = odrives.all_axes(|ax| ax.motor.set_control_mode(ControlMode::PositionControl, InputMode::PosFilter)).unwrap_all();
    //odrives.all_axes(|ax| ax.motor.set_input_pos(180 as f32 / 360 as f32));
    //odrives.all_axes(|ax| ax.motor.set_input_vel(10.0));
    //let c: Vec<Success<()>>  = odrives.axis(&0, |ax| ax.set_state(Idle)).unwrap();

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
