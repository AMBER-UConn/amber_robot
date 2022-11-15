use std::{f32::consts::PI, ops::Not};

use gilrs::{Gilrs, Button, Event};
use rustodrive::{odrivegroup::ODriveGroup, state::{AxisState::*, ControlMode, InputMode}, response::Success, utils::ResultAll};
use gilrs::Axis;


//pub mod gamepad;
fn set_speed(odrives: &ODriveGroup, sp: f32) -> () {
    let b: Vec<Success<()>> = odrives.all_axes(|ax| ax.motor.set_input_vel(sp)).unwrap_all();
}

fn set_pos(odrives: &ODriveGroup, po: f32) -> () {
    let b: Vec<Success<()>> = odrives.all_axes(|ax| ax.motor.set_input_pos(po)).unwrap_all();
}

pub fn controller(odrives: ODriveGroup) {


    //let odrv = ODriveGroup::new(can_read_WRITE, &[0,1]);

    let mut gilrs = Gilrs::new().unwrap();
    
    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    let mut speed: f32 = 0.0;

    let mut active_gamepad = None;
    

    let mut _last_deg = 0.0;
    let mut rotations = 0.0;

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            active_gamepad = Some(id);
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {

            let mut is_vel = true;

            is_vel = if gamepad.is_pressed(Button::South) { //South Button - POSITION MODE 
                let aaa: Vec<Success<()>> = odrives.all_axes(|ax| ax.motor.set_control_mode(ControlMode::PositionControl, InputMode::Passthrough)).unwrap_all();
                println!("POSITION CONTROL!");
                false
            } else {is_vel};

            is_vel = if gamepad.is_pressed(Button::West) { //West Button - VELOCITY MODE
                let aaa: Vec<Success<()>> = odrives.all_axes(|ax| ax.motor.set_control_mode(ControlMode::VelocityControl, InputMode::VelRamp)).unwrap_all();
                println!("VELOCITY CONTROL!");
                true
            } else {is_vel};
            
            if gamepad.is_pressed(Button::North){ //North Button - CLOSED LOOP
                println!("Closed Loop!");
                let aaa: Vec<Success<()>> = odrives.all_axes(|ax| ax.set_state(ClosedLoop)).unwrap_all();
            }
            
            if is_vel { // Velocity Mode
                if gamepad.is_pressed(Button::DPadUp) {
                    speed = speed.abs();
                    set_speed(&odrives, speed);
                }

                if gamepad.is_pressed(Button::DPadDown) {
                    speed = -speed.abs();
                    set_speed(&odrives, speed);
                }
                
                if gamepad.is_pressed(Button::DPadRight) {
                    speed = speed + (1.0/60.0);
                    set_speed(&odrives, speed);
                }
                
                if gamepad.is_pressed(Button::DPadLeft) {
                    speed = speed - (1.0/60.0);
                    set_speed(&odrives, speed);
                }

                if gamepad.is_pressed(Button::East) { //EAST Button sets velocity to 0
                    speed = 0.0;
                    set_speed(&odrives, speed);
                }
            }
            
            //println!("{}", is_vel);

            let factor = 1.0;
            
            let ls_x = gamepad.value(Axis::LeftStickX);
            let ls_y = gamepad.value(Axis::LeftStickY);

            // inverse_kinematics(factor*ls_x, factor*ls_y)
                      

    }
}
}