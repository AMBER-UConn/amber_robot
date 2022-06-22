use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

use socketcan::CANSocket;

use crate::messages::{ODriveMessage, ODriveResponse};

pub struct CANProxy {
    msg_receiver: Receiver<ODriveMessage>,
    thread_connections: HashMap<usize, Sender<ODriveResponse>>,

    awaiting_response: Vec<ODriveMessage>,
    socket: CANSocket,
}

impl CANProxy {
    pub fn new(
        can_device: &str,
        msg_receiver: Receiver<ODriveMessage>,
        thread_connections: HashMap<usize, Sender<ODriveResponse>>
    ) -> Self {
        // Initialize CANSocket
        let socket = CANSocket::open(can_device).expect("Could not open CAN at can1");

        Self {
            msg_receiver,
            thread_connections,
            socket,
            awaiting_response: vec![],
        }
    }

    /// Takes the next message off the channel and sends it to the CAN device
    fn send_to_CAN(&self) {}

    /// Receives responses from ODrive
    fn rcv_from_CAN(&self) {}

    /// it attempts to match and messages that are waiting to one that was received
    fn match_with_waiting(received_msg: &ODriveMessage) {

    }

    /// get the channel for a particular access to respond to
    fn thread_channel(&self, thread_id: usize) {}
}