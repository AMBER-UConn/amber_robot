use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;

use crate::cansocket::CANSocket;
use crate::messages::{ODriveMessage, ODriveResponse};

pub struct CANProxy {
    mpsc_channel: (Sender<ODriveMessage>, Receiver<ODriveMessage>),
    thread_connections: HashMap<usize, Sender<ODriveResponse>>,

    awaiting_response: Vec<ODriveMessage>,
    socket: CANSocket,
}

impl CANProxy {
    pub fn new(can_device: &str) -> Self {
        // Initialize CANSocket
        let socket = CANSocket::open(can_device).expect("Could not open CAN at can1");

        // Define the channel for the proxy here
        let mpsc_channel = channel::<ODriveMessage>();

        Self {
            mpsc_channel,
            socket,
            thread_connections: HashMap::new(),
            awaiting_response: vec![],
        }
    }

    pub fn register<T>(&mut self, thread: T) -> JoinHandle<()>
    where
        T: FnOnce(Sender<ODriveMessage>, Receiver<ODriveResponse>) + std::marker::Send + 'static,
    {
        // Thread <--- CANManager sends ODriveResponse
        let (thread_sender, thread_receiver) = channel::<ODriveResponse>();
        let send_to_proxy = self.mpsc_channel.0.clone();

        // Give the proxy the ability to read from threads
        // and send to the threads
        let mut max_id = match self.get_max_thread_id() {
            None => 0,
            Some(val) => val
        };
        self.thread_connections.insert(max_id, thread_sender);

        // Give the thread the ability to send to the proxy
        // and receive from the proxy
        std::thread::spawn(|| thread(send_to_proxy, thread_receiver))
    }

    /// Get the max thread ID that is being tracked
    fn get_max_thread_id(&self) -> Option<usize> {
        return self.thread_connections.keys().max().cloned();
    }

    /// Takes the next message off the channel and sends it to the CAN device
    fn send_to_CAN(&self) {}

    /// Receives responses from ODrive
    fn rcv_from_CAN(&self) {}

    /// it attempts to match and messages that are waiting to one that was received
    fn match_with_waiting(received_msg: &ODriveMessage) {}

    /// get the channel for a particular access to respond to
    fn thread_channel(&self, thread_id: usize) {}
}

#[cfg(test)]
mod tests {
    use crate::cansocket::CANSocket;

    fn test_register_thread() {
        
    }
}