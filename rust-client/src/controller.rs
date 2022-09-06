use std::{f32::consts::PI, ops::Not};

use gilrs::{Gilrs, Button, Event};
use rustodrive::{odrivegroup::ODriveGroup, state::{AxisState::*, ControlMode, InputMode}, response::Success, utils::ResultAll};
use gilrs::Axis;


//pub mod gamepad;

pub fn controller(odrives: ODriveGroup) {


    //let odrv = ODriveGroup::new(can_read_WRITE, &[0,1]);

    let mut gilrs = Gilrs::new().unwrap();
    
    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    let mut speed: f32 = 0.0;

    let mut active_gamepad = None;
    
    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            active_gamepad = Some(id);
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {

            fn check_neg(a: f32) -> bool {
                return (a == a.abs()).not();
            }

            let ls_x = gamepad.value(Axis::LeftStickX);
            let ls_y = gamepad.value(Axis::LeftStickY);
            let mut ls_rad = (ls_y / ls_x).atan();
            

            if check_neg(ls_x) && check_neg(ls_y) { //3rd q
                ls_rad += PI;
            }
            else if check_neg(ls_x) { //2nd q
                ls_rad += PI;
            }
            else if check_neg(ls_y) { //4th q
                ls_rad += 2.0*PI;
            }

            let mut ls_deg = ls_rad * (180.0/PI);

            println!("LS: {}", ls_deg);  

            let mut ls_rot = ls_deg / 360.0;

            if gamepad.is_pressed(Button::South) {
                println!("Button South is pressed (XBox - A, PS - X)");
            }
            if gamepad.is_pressed(Button::North){
                println!("Closed Loop!");
                let aaa: Vec<Success<()>> = odrives.all_axes(|ax| ax.set_state(ClosedLoop)).unwrap_all();
            }

            if gamepad.is_pressed(Button::DPadUp) {
                speed = speed.abs();
            }

            if gamepad.is_pressed(Button::DPadDown) {
                speed = -speed.abs();
            }
            
            if gamepad.is_pressed(Button::DPadRight) {
                speed = speed + (1.0/60.0);
            }
            
            if gamepad.is_pressed(Button::DPadLeft) {
                speed = speed - (1.0/60.0);
            }

            if gamepad.is_pressed(Button::East) {
                speed = 0.0;
            }

            let b: Vec<Success<()>> = odrives.all_axes(|ax| ax.motor.set_input_vel(speed)).unwrap_all();
                      

    }
}
}