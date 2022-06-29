
use std::io;
use socketcan::{CANSocketOpenError, CANFrame};
use crate::cfg_match;
use rand::seq::SliceRandom;

cfg_match! {
    feature = "mock-socket" => {

        /// Mock implementation
        pub(crate) struct CANSocket {
            received: Vec<CANFrame>,
        }

        impl CANSocket {
            pub fn open(ifname: &str) -> Result<Self, CANSocketOpenError> {
                Ok(CANSocket{ received: Vec::new() })
            }

            pub fn write_frame(&mut self, frame: &CANFrame) -> io::Result<()> {
                self.received.push(frame.clone());
                Ok(())
            }

            pub fn read_frame(&self) -> io::Result<CANFrame> {
                // For the sake of testing purposes, we return an Io Error that
                // indicates this method would be blocked if it waited. In actuality reading and writing occurs
                // in parallel so our code would work fine otherwise
                match self.received.choose(&mut rand::thread_rng()) {
                    Some(item) => return Ok(item.clone()),
                    None => return Err(io::Error::new(io::ErrorKind::WouldBlock, "no messages available")),
                }
            }
        }   
    },
    other => {
        pub(crate) use socketcan::CANSocket;
    },
}