
use std::io;
use socketcan::{CANSocketOpenError, CANFrame};
use crate::{cfg_match, messages::ODriveCANFrame, commands::ODriveCommand};
use rand::seq::SliceRandom;

cfg_match! {
    feature = "mock-socket" => {

        /// Mock implementation
        pub(crate) struct CANSocket {
            waiting: Vec<CANFrame>,
        }

        impl CANSocket {
            pub fn open(ifname: &str) -> Result<Self, CANSocketOpenError> {
                Ok(CANSocket{ waiting: Vec::new() })
            }

            pub fn write_frame(&mut self, frame: &CANFrame) -> io::Result<()> {
                // The odrive only responds to Read commands, not Write. This imitates that
                match ODriveCANFrame::from_can(&frame).cmd {
                    ODriveCommand::Read(_) => self.waiting.push(frame.clone()),
                    ODriveCommand::Write(_) => {},
                }

                Ok(())
            }

            pub fn read_frame(&self) -> io::Result<CANFrame> {
                // For the sake of testing purposes, we return an Io Error that
                // indicates this method would be blocked if it waited. In actuality reading and writing occurs
                // in parallel so our code would work fine otherwise
                match self.waiting.choose(&mut rand::thread_rng()) {
                    Some(item) => { 
                        let mut cloned_frame = ODriveCANFrame::from_can(&item);
                        
                        // We use [99; 8] just to have a response that is not the same as the request
                        cloned_frame.data = [99; 8];

                        // The CAN response does not respond with RTR enabled
                        return Ok(cloned_frame.to_can(false))
                    
                    },
                    None => return Err(io::Error::new(io::ErrorKind::WouldBlock, "no messages available")),
                }
            }
        }   
    },
    other => {
        pub(crate) use socketcan::CANSocket;
    },
}