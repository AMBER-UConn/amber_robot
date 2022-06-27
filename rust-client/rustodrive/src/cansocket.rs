
use std::io;
use socketcan::{CANSocketOpenError, CANFrame};
use crate::cfg_match;

cfg_match! {
    feature = "mock-socket" => {
        /// Mock implementation
        pub(crate) struct CANSocket {}

        impl CANSocket {
            pub fn open(ifname: &str) -> Result<Self, CANSocketOpenError> {
                Ok(CANSocket{})
            }

            pub fn write_frame(&self, frame: &CANFrame) -> io::Result<()> {
                Ok(())
            }

            pub fn read_frame(&self) -> io::Result<CANFrame> {
                Ok(CANFrame::new(0, &[0], false, false).unwrap())
            }
        }   
    },
    other => {
        pub(crate) use socketcan::CANSocket;
    },
}