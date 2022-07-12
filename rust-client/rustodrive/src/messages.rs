use crate::commands;
use crate::commands::ODriveCommand;
use socketcan::CANFrame;

pub type CANRequest = ODriveCANFrame;
pub type CANResponse = ODriveCANFrame;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ODriveCANFrame {
    pub axis: u32,
    pub cmd: ODriveCommand,
    pub data: [u8; 8],
}

impl ODriveCANFrame {
    const AXIS_BITS: u32 = 5;

    pub fn to_can(&self, rtr: bool) -> socketcan::CANFrame {
        let id = self.axis << Self::AXIS_BITS | self.get_cmd_id();
        return socketcan::CANFrame::new(id, &self.data, rtr, false).unwrap(); // ODrive rquires the RTR bitset be enabled for call/response
    }

    fn get_cmd_id(&self) -> u32 {
        match self.cmd {
            ODriveCommand::Read(cmd) => cmd as u32,
            ODriveCommand::Write(cmd) => cmd as u32,
        }
    }

    fn to_cmd(can_id: u32) -> ODriveCommand {
        // 0x1F is 00011111 in binary. Take the last 5 bits to get the command
        let cmd_id = can_id & 0x1F;

        // Then try converting to a write command
        match TryInto::<commands::Write>::try_into(cmd_id) {
            Ok(cmd) => return ODriveCommand::Write(cmd),
            Err(_) => {}
        }

        // Try first converting to a read command
        match TryInto::<commands::Read>::try_into(cmd_id) {
            Ok(cmd) => return ODriveCommand::Read(cmd),
            Err(_) => panic!("CAN ID {} not able to be converted to a command", can_id),
        }
    }

    pub fn from_can(frame: &CANFrame) -> Self {
        // Get the first 5 bits
        let axis = frame.id() >> Self::AXIS_BITS;
        let cmd = Self::to_cmd(frame.id());

        ODriveCANFrame {
            axis,
            cmd,
            data: frame.data().try_into().expect(
                "Error with CANFrame. data() returned slice that could not be coerced into [u8; 8]",
            ),
        }
    }

    // If the command and axis IDs match, then it must be the response
    pub fn is_response(&self, other: &ODriveCANFrame) -> bool {
        self.axis == other.axis && self.cmd == other.cmd
    }
}
#[derive(Clone, PartialEq, Debug)]
pub struct ODriveMessage {
    pub thread_name: &'static str,
    pub body: ODriveCANFrame,
}


#[cfg(test)]
mod tests {
    use crate::{
        commands::{ODriveCommand, Read, Write},
        messages::{CANRequest, CANResponse},
    };

    use super::ODriveCANFrame;

    #[test]
    fn test_conversion_to_frame() {
        // Tests if converting from the CAN frame and back retains the right data
        let msg1 = CANRequest {
            axis: 0x1,
            cmd: ODriveCommand::Write(Write::SetInputPosition), // this is cmd id 0x0C
            data: [0; 8],
        };

        let msg2 = CANRequest {
            axis: 0x293874,
            cmd: ODriveCommand::Read(Read::GetEncoderCount), // this is cmd id 0x0C
            data: [0; 8],
        };

        // Calculated by hand. See this example https://docs.odriverobotics.com/v/latest/can-protocol.html#can-frame
        let rtr_enabled = true;
        let can_frame = msg1.to_can(rtr_enabled);
        assert_eq!(can_frame.id(), 0x2C);
        assert_eq!(can_frame.data(), [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(can_frame.is_rtr(), rtr_enabled);

        // Test it converts back properly
        assert_eq!(msg1, msg1.clone()); // test that equals works
        assert_eq!(msg1, ODriveCANFrame::from_can(&can_frame));

        // test that conversion works with read messages
        // Converting from CANFrame to ODriveCANFrame ignores
        //the rtr bit so either true/false is fine
        assert_eq!(msg2, ODriveCANFrame::from_can(&msg2.to_can(true)));
    }

    #[test]
    #[should_panic]
    fn test_command_not_found() {
        ODriveCANFrame::to_cmd(0xFFFF);
    }

    #[test]
    fn test_is_response() {
        let msg1 = CANRequest {
            axis: 0x1,
            cmd: ODriveCommand::Write(Write::SetInputPosition), // this is cmd id 0x0C
            data: [0; 8],
        };
        let fake_response = CANResponse {
            axis: 0x1,
            cmd: ODriveCommand::Write(Write::SetInputPosition), // this is cmd id 0x0C
            data: [1; 8], // the data has changed but the rest is the same
        };
        assert_eq!(msg1.is_response(&fake_response), true);
    }
}
