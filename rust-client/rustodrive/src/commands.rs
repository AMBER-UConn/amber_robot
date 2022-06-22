/// Documentation: https://docs.odriverobotics.com/v/latest/can-protocol.html#messages
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ODriveCommand {
    Heartbeat = 0x001,
    EStop = 0x002,
    MotorError = 0x003,
    EncoderError = 0x004,
    SensorlessError = 0x005,
    SetAxisNodeID = 0x006,
    SetAxisRequestedState = 0x007,
    // SetAxisStartupConfig **Not yet implemented in ODrive according to documentation**
    GetEncoderEstimates = 0x009,
    GetEncoderCount = 0x00A,
    SetControllerModes = 0x00B,
    SetInputPosition = 0x00C,
    SetInputVelocity = 0x00D,
    SetInputTorque = 0x00E,
    SetLimits = 0x00F,
    StartAnticogging = 0x010,
    SetTrajVelocityLim = 0x011,
    SetTrajAccelLim = 0x012,
    SetTrajInertia = 0x013,
    GetIQSetpoint = 0x014,
    GetSensorlessEstimates = 0x015,
    RebootODrive = 0x016,
    GetVBusVoltage = 0x017,
    ClearErrors = 0x018,
    SetLinearCount = 0x019,
    SetPositionGain = 0x01A,
    SetVelocityGain = 0x01B,
}

/// Documentation: https://docs.odriverobotics.com/v/latest/fibre_types/com_odriverobotics_ODrive.html?highlight=axisstate#ODrive.Axis.AxisState
pub enum ODriveAxisState {
    Undefined = 0x0,
    Idle = 0x1,
    StartupSequence = 0x2,
    FullCalibrationSequence = 0x3,
    MotorCalibration = 0x4,
    EncoderIndexSearch = 0x5,
    EncoderOffsetCalib = 0x7,
    LockinSpin = 0x9,
    EncoderDirFind = 0xA,
    Homing = 0xB,
    EncoderHallPolarityCalib = 0xC,
    EncoderHallPhaseCalib = 0xD,
}
