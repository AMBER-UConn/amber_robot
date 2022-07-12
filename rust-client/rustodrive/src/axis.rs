use crate::{commands::{ODriveAxisState, ODriveCommand, Write}, messages::ODriveCANFrame};


pub type AxisID = usize;
pub struct Axis<'a> {
    id: &'a AxisID,
    motor: Motor<'a>,
    encoder: Encoder<'a>,
}

impl<'a> Axis<'a> {
    pub fn new(id: &'a AxisID) -> Self {
        Axis {
            id,
            motor: Motor::new(id),
            encoder: Encoder::new(id),
        }
    }

    pub fn set_state(&self, state: ODriveAxisState) -> ODriveCANFrame {
        ODriveCANFrame { axis: *self.id as u32, cmd: ODriveCommand::Write(Write::SetAxisRequestedState), data: [state as u8, 0, 0, 0, 0, 0, 0, 0] }
    }
}



struct Encoder<'a> {
    id: &'a AxisID,
}
impl<'a> Encoder<'a> {
    pub fn new(id: &'a AxisID) -> Self {
        Encoder { id }
    }
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

struct Motor<'a> {
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
