use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;

use crate::cansocket::CANSocket;
use crate::messages::{ODriveMessage, ODriveResponse};

type ThreadConnection = (JoinHandle<()>, Sender<ODriveResponse>);

pub enum ProxyError {
    ThreadFailedJoin
}

pub struct CANProxy {
    mpsc_channel: (Sender<ODriveMessage>, Receiver<ODriveMessage>),
    thread_connections: HashMap<usize, ThreadConnection>,

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

    pub fn register<F>(&mut self, func: F) where F: FnOnce(Sender<ODriveMessage>, Receiver<ODriveResponse>) + std::marker::Send + 'static,
    {
        // Thread <--- CANManager sends ODriveResponse
        let (thread_sender, thread_receiver) = channel::<ODriveResponse>();
        let send_to_proxy = self.mpsc_channel.0.clone();

        // Give the proxy the ability to read from threads
        // and send to the threads
        let max_id = match self.max_thread_id() {
            None => 0,
            Some(val) => val + 1
        };

        // Give the thread the ability to send to the proxy
        // and receive from the proxy
        let thread_handle = std::thread::spawn(|| func(send_to_proxy, thread_receiver));
        
        // Add the thread and keep track of it
        self.thread_connections.insert(max_id, (thread_handle, thread_sender));
    }

    pub fn unregister(&mut self, thread_id: &usize) -> Result<(), ProxyError> {
        if self.thread_connections.contains_key(thread_id) {
            let (thread_handle, _sender) = self.thread_connections.remove(thread_id).unwrap();
            match thread_handle.join() {
                Err(_e) => return Err(ProxyError::ThreadFailedJoin),
                Ok(()) => return Ok(())
            }
        } else {
            panic!("Cannot unregister thread ID that doesn't exist")
        }
    }

    /// Get the max thread ID that is being tracked
    fn max_thread_id(&self) -> Option<usize> {
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
    use std::thread::JoinHandle;
    use crate::cansocket::CANSocket;
    use super::CANProxy;

    #[test]
    fn test_register_thread() {
        let mut can_proxy = CANProxy::new("fakecan");

        for i in (0..3) {
            can_proxy.register(|send_to_proxy, thread_receiver| {})
        }
        assert_eq!(can_proxy.thread_connections.len(), 3);
        
        can_proxy.unregister(&1);
        can_proxy.register(|send, receive| {});

        // Check that the max_id = 3 and the length is 3
        assert_eq!(can_proxy.thread_connections.len(), 3);
        assert_eq!(can_proxy.max_thread_id().unwrap(), 3);
    }
}