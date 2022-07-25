use socketcan::CANFrame;

use crate::{
    commands::{ODriveAxisState, ODriveCommand::{Read, Write}, WriteComm::*, ReadComm::*},
    messages::{ticket, CANRequest, ODriveCANFrame},
    utils::{combine_data, float_to_data},
};

pub type AxisID = usize;

/// This struct contains methods that can generate common [`ODriveCANFrame`] configurations.
/// The [`Motor`] and [`Encoder`] objects are publicly accessible and define their own
/// frame-generating methods.
pub struct Axis<'a> {
    id: &'a AxisID,
    pub motor: Motor<'a>,
    pub encoder: Encoder<'a>,
}

impl<'a> Axis<'a> {
    pub fn new(id: &'a AxisID) -> Self {
        Axis {
            id,
            motor: Motor::new(id),
            encoder: Encoder::new(id),
        }
    }

    /// This generates the command to set the state for the `Axis` object in question
    pub fn set_state(&self, state: ODriveAxisState) -> CANRequest {
        ticket(
            *self.id,
            Write(SetAxisRequestedState),
            [state as u8, 0, 0, 0, 0, 0, 0, 0],
        )
        //CANRequest { axis: *self.id as u32, cmd: ODriveCommand::Write(Write::SetAxisRequestedState), data: [state as u8, 0, 0, 0, 0, 0, 0, 0] }
    }

    //pub fn set_control_mode
}

pub struct Encoder<'a> {
    id: &'a AxisID,
}
impl<'a> Encoder<'a> {
    pub fn new(id: &'a AxisID) -> Self {
        Encoder { id }
    }
    fn get_error() {
        unimplemented!()
    }
    fn get_count(&self) -> CANRequest {
        return ticket(*self.id, Read(GetEncoderCount), [0; 8]);
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

pub struct Motor<'a> {
    id: &'a AxisID,
}
impl<'a> Motor<'a> {
    pub fn new(id: &'a AxisID) -> Self {
        Motor { id }
    }

    fn get_error() {
        unimplemented!()
    }
    fn get_sensorless_error() {
        unimplemented!()
    }

    fn set_node_id() {
        unimplemented!()
    }
    fn set_control_mode() {
        unimplemented!()
    }

    pub fn set_input_pos(&self, rot: f32) -> CANRequest {
        let data = combine_data(float_to_data(rot), [0; 4]);
        {
            ticket(*self.id, Write(SetInputPosition), data)
        }
    }
    pub fn set_input_vel(&self, speed: f32) -> CANRequest {
        let data = combine_data(float_to_data(speed), [0; 4]);
        {
            ticket(*self.id, Write(SetInputVelocity), data)
        }
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
