use rustodrive::{
    canproxy::CANProxy,
    commands::{ControlMode, ControlMode::*, InputMode, InputMode::*, ODriveAxisState::*, WriteComm},
    messages::CANRequest,
    odrivegroup::ODriveGroup,
    threads::ReadWriteCANThread,
};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error, io::stdin};

fn odrive_main(can_read_write: ReadWriteCANThread) {
    fn input(txt: &'static str) -> String {
        let mut out = String::new();
        println!("{}", txt);
        stdin().read_line(&mut out).unwrap();
        return out.trim().to_string();
    }

    let odrives = ODriveGroup::new(can_read_write, &[0, 1, 2, 3, 4, 5]);
    let mut is_closed_loop = false;
    let mut inp = String::new();

    while true {
        //println!();
        //stdin().read_line(&mut inp).unwrap();
        inp = input("Input (C - Toggle Closed Loop, V - Input Velocity, P - Input Position, CM - Control Mode / Input Mode) > ");
        //println!("{}", inp.to_uppercase());
        match inp.to_uppercase().as_str() {
            "C" => {
                if (is_closed_loop) {
                    odrives.all_axes(|ax| ax.set_state(Idle));
                    is_closed_loop = false;
                } else {
                    odrives.all_axes(|ax| ax.set_state(ClosedLoop));
                    is_closed_loop = true;
                }
            }

            "V" => {
                let inp_vel: f32 = input("Input Velocity > ")
                    .parse::<f32>()
                    .unwrap_or_default();
                odrives.all_axes(|ax| ax.motor.set_input_vel(inp_vel));
            }

            "P" => {
                let inp_pos: f32 = input("Input Velocity > ").parse::<f32>().unwrap();
                odrives.all_axes(|ax| ax.motor.set_input_pos(inp_pos));
            }

            "CM" => {
                let inp_cm: u32 =
                    input("Input Control Mode (2 - Velocity Control, 3 - Position Control)")
                        .parse::<u32>()
                        .unwrap_or_default();
                let inp_im: u32 =
                    input("Input Control Mode (1 - Passthrough, 2 - VelRamp, 3 - PosFilter)")
                        .parse::<u32>()
                        .unwrap_or_default();
                
                odrives.all_axes(move |ax| {
                    let CM = TryInto::<ControlMode>::try_into(inp_cm)
                        .unwrap_or(ControlMode::VelocityControl);
                    let IM = TryInto::<InputMode>::try_into(inp_im)
                        .unwrap_or(InputMode::Passthrough);
                    ax.motor.set_control_mode(CM, IM)
                });}

            _ => println!("Not a valid command."),
    }}
    //odrives.all_axes(|ax| ax.set_state(ClosedLoop));

    //odrives.all_axes(|ax| ax.motor.set_control_mode(PositionControl));
    //odrives.all_axes(|ax| ax.motor.set_input_pos(180 as f32 / 360 as f32));
    //odrives.all_axes(|ax| ax.motor.set_input_vel(10.0));

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
