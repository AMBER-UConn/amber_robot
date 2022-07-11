use std::collections::HashMap;

use crate::{
    messages::{ODriveResponse, ODriveCANFrame},
    threads::ReadWriteCANThread, commands::{ODriveCommand, ODriveAxisState, Write},
};

pub type AxisID = usize;

pub struct ODriveGroup<'a> {
    can: ReadWriteCANThread,
    axes: HashMap<&'a AxisID, Axis<'a>>,
}

impl<'a> ODriveGroup<'a> {
    pub fn new(can: ReadWriteCANThread, axis_ids: &'static [AxisID]) -> Self {
        ODriveGroup {
            axes: axis_ids.iter().map(|id| (id, Axis::new(id))).collect(),
            can,
        }
    }

    pub fn all_axes<F>(&self, f: F) -> Vec<ODriveResponse>
    where
        F: FnMut(&Axis) -> ODriveCANFrame,
    {
        let requests = self.axes.values().map(|ax| f(ax)).collect();
        self.can.request_many(requests)
    }

    pub fn axis<F>(&self, axis_id: &AxisID, f: F) -> ODriveResponse
    where
        F: FnOnce(&Axis) -> ODriveCANFrame,
    {
        self.can.request(f(self.get_axis(axis_id)))
    }
    
    fn get_axis(&self, id: &AxisID) -> &Axis {
        match self.axes.get(id) {
            Some(axis) => axis,
            None => panic!("Cannot retrieve axis {} that doesn't exist!", id)
        }
    }
}

struct Axis<'a> {
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

pub trait ManyResponses {
    fn expect_bodies(self, msg: &str) -> Vec<ODriveCANFrame>;
}
impl ManyResponses for Vec<ODriveResponse> {

    /// This method calls .expect() on all responses. 
    /// This will panic if called on a response that was
    /// read only (ex: Heartbeat)
    fn expect_bodies(self, msg: &str) -> Vec<ODriveCANFrame> {
        let mut frames = Vec::new();

        for res in self.into_iter() {
            match res {
                ODriveResponse::Response(body) => frames.push(body.expect(msg)),
                ODriveResponse::ReqReceived(_) => panic!("Write requests do not return a response body")
            }
        }
        frames
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
