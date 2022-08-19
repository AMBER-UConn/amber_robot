use rustodrive::{
    state::{
        ControlMode, InputMode, AxisState::*,
    },
    odrivegroup::ODriveGroup,
};
use std::io::stdin;

pub fn ui_start(odrives: ODriveGroup) {
    fn input(txt: &str) -> String {
        let mut out = String::new();
        println!("{}", txt);
        stdin().read_line(&mut out).unwrap();
        return out.trim().to_string();
    }

    let mut is_closed_loop = false;

    loop {
        let disp_txt = format!("Input (C - Toggle Closed Loop ({}), V - Input Velocity, P - Input Position, CM - Control Mode / Input Mode) > ", is_closed_loop);
        match input(disp_txt.as_str()).to_uppercase().as_str() {

            // Toggle Control Mode
            "C" => {
                if is_closed_loop {
                    odrives.all_axes(|ax| ax.set_state(Idle));
                    is_closed_loop = false;
                } else {
                    odrives.all_axes(|ax| ax.set_state(ClosedLoop));
                    is_closed_loop = true;
                }
            }
            
            // Change Input Velocity
            "V" => {
                let inp_vel: f32 = input("Input Velocity > ")
                    .parse::<f32>()
                    .unwrap_or_default();
                odrives.all_axes(|ax| ax.motor.set_input_vel(inp_vel));
            }

            // Change Input Position
            "P" => {
                let inp_pos: f32 = input("Input Velocity > ").parse::<f32>().unwrap();
                odrives.all_axes(|ax| ax.motor.set_input_pos(inp_pos));
            }

            // Change Control Mode & Input Mode
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
                    let control_mode = 
                        TryInto::<ControlMode>::try_into(inp_cm).unwrap_or(ControlMode::VelocityControl);
                    let input_mode =
                        TryInto::<InputMode>::try_into(inp_im).unwrap_or(InputMode::Passthrough);
                    ax.motor.set_control_mode(control_mode, input_mode)
                });
            }

            "I" => {odrives.all_axes(|ax| ax.set_state(EncoderIndexSearch));},

            // Quit
            "Q" => {
                println!("Quitting...");
                odrives.all_axes(|ax| ax.set_state(Idle));
                std::process::exit(0);
            }

            // Invalid Command Handler
            _ => println!("Not a valid command."),
        }
    }
}
