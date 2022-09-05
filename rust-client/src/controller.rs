use gilrs::{Gilrs, Button, Event};
use rustodrive::{odrivegroup::ODriveGroup, state::{AxisState::*, ControlMode, InputMode}, response::Success, utils::ResultAll};

//pub mod gamepad;

pub fn controller(odrives: ODriveGroup) {


    //let odrv = ODriveGroup::new(can_read_WRITE, &[0,1]);

    let mut gilrs = Gilrs::new().unwrap();
    
    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    let mut speed = 0.0;

    let mut active_gamepad = None;
    
    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            active_gamepad = Some(id);
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            if gamepad.is_pressed(Button::South) {
                println!("Button South is pressed (XBox - A, PS - X)");
            }
            if gamepad.is_pressed(Button::North){
                println!("Closed Loop!");
                let aaa: Vec<Success<()>> = odrives.all_axes(|ax| ax.set_state(ClosedLoop)).unwrap_all();
            }

            if gamepad.is_pressed(Button::DPadUp) {
                let b: Vec<Success<()>> = odrives.all_axes(|ax| ax.motor.set_input_vel(10.0)).unwrap_all();
            }

            if gamepad.is_pressed(Button::DPadDown) {
                let b: Vec<Success<()>> = odrives.all_axes(|ax| ax.motor.set_input_vel(-10.0)).unwrap_all();
            }


    }
}
}