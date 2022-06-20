use socketcan::{CANSocket, CANFrame};

struct Encoder;
impl Encoder {
    fn get_error() { unimplemented!() }

    fn get_count() { unimplemented!() }
    fn get_estimate() { unimplemented!() }
    fn set_linear_count() { unimplemented!() }
}

struct Trajectory;
impl Trajectory {
    fn set_traj_vel_limit() { unimplemented!() }
    fn set_traj_accel_limit() { unimplemented!() }
    fn set_traj_inertia() { unimplemented!() }
}

struct Axis {
    id: usize,
    motor: Motor,
    encoder: Encoder,
}

struct Motor {}

impl Motor {
    fn get_error() { unimplemented!() }
    fn get_sensorless_error() { unimplemented!() }

    fn set_node_id() { unimplemented!() }
    fn set_state() { unimplemented!() }
    fn set_control_mode() { unimplemented!() }

    fn set_input_pos() { unimplemented!() }
    fn set_input_vel() { unimplemented!() }
    fn set_input_torque() { unimplemented!() }

    fn set_limits() { unimplemented!() } // velocity and current limit

    
    fn get_iq_setpoint() { unimplemented!() }

    fn set_position_gain() { unimplemented!() }
    fn set_vel_gain() { unimplemented!() }

    fn get_sensorless_estimates() { unimplemented!() }

}

struct ODrive;
impl ODrive {
    fn start_anticogging() { unimplemented!() }
    fn start_calibration() { unimplemented!() }

    fn reboot() { unimplemented!() }
    fn get_vbus_voltage() { unimplemented!() }
    fn clear_errors() { unimplemented!() }
    fn has_heartbeat() { unimplemented!() }
}

fn main() {

}