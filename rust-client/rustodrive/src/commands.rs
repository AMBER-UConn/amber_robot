use crate::{back_to_enum};

back_to_enum! {
    #[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
    pub enum ReadComm {
        Heartbeat = 0x001,
        MotorError = 0x003,
        EncoderError = 0x004,
        SensorlessError = 0x005,
        GetEncoderEstimates = 0x009,
        GetEncoderCount = 0x00A,
        GetIQSetpoint = 0x014,
        GetSensorlessEstimates = 0x015,
        GetVBusVoltage = 0x017,
    }
}

//impl ReadComm {
//    pub fn to_msg(axis: u32, cmd: Self) -> ODriveCANFrame {
//        ODriveCANFrame { axis, cmd: ODriveCommand::Read(cmd), data: [0; 8] }
//    }
//}


back_to_enum! {
    #[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
    pub enum WriteComm {
        EStop = 0x002,

        SetAxisNodeID = 0x006,
        SetAxisRequestedState = 0x007,
        // SetAxisStartupConfig **Not yet implemented in ODrive according to documentation**
        SetControllerMode = 0x00B,
        SetInputPosition = 0x00C,
        SetInputVelocity = 0x00D,
        SetInputTorque = 0x00E,
        SetLimits = 0x00F,
        StartAnticogging = 0x010,
        SetTrajVelocityLim = 0x011,
        SetTrajAccelLim = 0x012,
        SetTrajInertia = 0x013,
        RebootODrive = 0x016,
        ClearErrors = 0x018,
        SetLinearCount = 0x019,
        SetPositionGain = 0x01A,
        SetVelocityGain = 0x01B,
    }
}

//impl WriteComm {
//    pub fn to_msg(axis: u32, cmd: Self, data: [u8; 8]) -> ODriveCANFrame {
//        ODriveCANFrame { axis, cmd: ODriveCommand::Write(cmd), data }
//    }
//}


/// Documentation: <https://docs.odriverobotics.com/v/latest/can-protocol.html#messages>
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum ODriveCommand {
    Read(ReadComm),
    Write(WriteComm),
}

back_to_enum! {
    /// Documentation: <https://docs.odriverobotics.com/v/latest/fibre_types/com_odriverobotics_ODrive.html?highlight=axisstate#ODrive.Axis.AxisState>
    pub enum ODriveAxisState {
        Undefined = 0x0,
        Idle = 0x1,
        StartupSequence = 0x2,
        FullCalibrationSequence = 0x3,
        MotorCalibration = 0x4,
        EncoderIndexSearch = 0x5,
        EncoderOffsetCalib = 0x7,
        ClosedLoop = 0x8,
        LockinSpin = 0x9,
        EncoderDirFind = 0xA,
        Homing = 0xB,
        EncoderHallPolarityCalib = 0xC,
        EncoderHallPhaseCalib = 0xD,
    }
}


//https://docs.odriverobotics.com/v/latest/fibre_types/com_odriverobotics_ODrive.html#ODrive.Controller.ControlMode
back_to_enum!{
    pub enum ControlMode {
        VoltageControl = 0x0,
        TorqueControl = 0x1,
        VelocityControl = 0x2,
        PositionControl = 0x3,
    }
}
back_to_enum!{
    pub enum InputMode {
        Inactive = 0x0,
        Passthrough = 0x1,
        VelRamp = 0x2,
        PosFilter = 0x3,
        MixChannels = 0x4,
        TrapTraj = 0x5,
        TorqueRamp = 0x6,
        Mirror = 0x7,
        Tuning = 0x8,
    }
}
