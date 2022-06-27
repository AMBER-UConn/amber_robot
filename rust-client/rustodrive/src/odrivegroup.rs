use std::sync::mpsc::{Receiver, Sender};

use socketcan::{CANFrame, CANSocket};

use crate::{
    commands::{self, ODriveAxisState},
};

pub struct ODriveGroup {
    axis_ids: &'static [usize],
}

impl ODriveGroup {
    pub fn new() -> Self {
        todo!()
    }
}

pub fn test_motor_calib() {
    let socket = CANSocket::open("can1").expect("Could not open CAN at can1");

    let axis = 0x0;
    let command = commands::Write::SetAxisRequestedState as u32;
    let state = ODriveAxisState::FullCalibrationSequence as u8;
    let frame = CANFrame::new(
        axis << 5 | command,
        &[state, 0, 0, 0, 0, 0, 0, 0],
        false,
        false,
    )
    .unwrap();
    println!("attempting to calibrate motor");

    match socket.write_frame(&frame) {
        Ok(()) => println!("Frame was sent!"),
        Err(error) => panic!("an error occurred with sending the can command"),
    }

    // Wait for response frame
    loop {
        // socket.set_filter(&[CANFilter::new(axis << 5 | command, (1 << 8) - 1).unwrap()]);

        let response = socket.read_frame().unwrap();
        println!("command {:#?}", response);
        // println!("The command was received successfully!");
    }
}

struct Encoder;
impl Encoder {
    fn get_error() {
        unimplemented!()
    }
    fn get_count() {
        unimplemented!()
    }
    fn get_estimate() {
        unimplemented!()
    }
    fn set_linear_count() {
        unimplemented!()
    }
}

struct Trajectory;
impl Trajectory {
    fn set_traj_vel_limit() {
        unimplemented!()
    }
    fn set_traj_accel_limit() {
        unimplemented!()
    }
    fn set_traj_inertia() {
        unimplemented!()
    }
}

struct Axis {
    id: usize,
    motor: Motor,
    encoder: Encoder,
}

struct Motor;
impl Motor {
    fn get_error() {
        unimplemented!()
    }
    fn get_sensorless_error() {
        unimplemented!()
    }

    fn set_node_id() {
        unimplemented!()
    }
    fn set_state() {
        unimplemented!()
    }
    fn set_control_mode() {
        unimplemented!()
    }

    fn set_input_pos() {
        unimplemented!()
    }
    fn set_input_vel() {
        unimplemented!()
    }
    fn set_input_torque() {
        unimplemented!()
    }

    fn set_limits() {
        unimplemented!()
    } // velocity and current limit

    fn get_iq_setpoint() {
        unimplemented!()
    }

    fn set_position_gain() {
        unimplemented!()
    }
    fn set_vel_gain() {
        unimplemented!()
    }

    fn get_sensorless_estimates() {
        unimplemented!()
    }
}
