use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};

use crate::constants::ODriveCommand;

// TODO make an error type
pub struct CANManager {
    thread_receiver: Receiver<ODriveMessage>,
    thread_senders: HashMap<usize, Sender<ODriveResponse>>,

    waiting: Vec<ODriveMessage>
}

impl CANManager {
    pub fn new(receiver: Receiver<ODriveMessage>, senders: HashMap<usize, Sender<ODriveResponse>>) -> Self {
        Self {
            thread_receiver: receiver,
            thread_senders: senders,
            waiting: vec![]
        }
    }

    fn send_commands(&self) {

    }

    fn receive_commands(&self) {

    }

    /// it attempts to match and messages that are waiting to one that was received
    fn match_messages() {

    }

    /// get the channel for a particular access to respond to
    fn get_axis_channel(&self, axis_id: usize) {
        
    }
}

pub struct ODriveMessage {
    axis_id: usize,
    command: ODriveCommand,
    data: [u8; 8]
}

impl ODriveMessage {
    fn can_id(&self) -> u16 {
        return (self.axis_id as u16) << 5 | (self.command.clone() as u16);
    }
}

pub struct ODriveResponse {}