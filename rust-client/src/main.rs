use rustodrive::{
    canproxy::CANProxy,
    odrivegroup::ODriveGroup,
    threads::ReadWriteCANThread, casts::Heartbeat, response::{Success}, utils::ResultAll,
};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error};

pub mod test_ui;

fn init_motors(odrv: &ODriveGroup) {
    //odrv.all_axes(|ax| ax.set_state(EncoderIndexSearch));
    std::thread::sleep_ms(2000);
}

// TODO update documentation to reflect new changes
fn odrive_main(can_read_write: ReadWriteCANThread) {
    let odrives = ODriveGroup::new(can_read_write, &[0, 1, 2, 3, 4, 5]);

    init_motors(&odrives);

    // test_ui::ui_start(odrives);
    // odrives.all_axes(|ax| ax.motor.set_control_mode(ControlMode::VelocityControl, InputMode::VelRamp));
    // odrives.all_axes(|ax| ax.set_state(ClosedLoop));
    // odrives.all_axes(|ax| ax.motor.set_input_pos(180 as f32 / 360 as f32));
    // odrives.all_axes(|ax| ax.motor.set_input_vel(10.0));
    // odrives.axis(&0, |ax| ax.set_state(Idle)).unwrap();
    
    let heartbeat: Vec<Success<Heartbeat>> = odrives.all_axes(|ax| ax.get_heartbeat()).unwrap_all();
    let heartbeat: Success<Heartbeat> = odrives.axis(&1, |ax| ax.get_heartbeat()).unwrap();

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
