use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;

use socketcan::CANFrame;

use crate::cansocket::CANSocket;
use crate::messages::{ODriveMessage, ODriveResponse};
use crate::threads::{ReadOnlyCANThread, ReadWriteCANThread};

type ThreadConnection = (JoinHandle<()>, Sender<ODriveResponse>);
type ThreadID = &'static str;
pub enum ProxyError {
    ThreadFailedJoin,
    ThreadDuplicateID,
}

pub struct CANProxy {
    mpsc_channel: (Sender<ODriveMessage>, Receiver<ODriveMessage>),
    threads: HashMap<ThreadID, ThreadConnection>,
    rw_thread: Option<ThreadID>, // There can only be one read and write thread at a time. Store the identifier in here

    listeners: Vec<ODriveMessage>,
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
            rw_thread: None,
            threads: HashMap::new(),
            listeners: vec![],
        }
    }
    pub fn register_rw<F>(&mut self, thread_name: &'static str, thread_func: F)
    where
        F: FnOnce(ReadWriteCANThread) + std::marker::Send + 'static,
    {
        match self.rw_thread {
            Some(thread_id) => {
                panic!("Cannot register more than 1 thread to have write access to CAN device")
            }
            None => {
                self.rw_thread = Some(thread_name);
                self.register(thread_name, move |thread_requester, thread_receiver| {
                    thread_func(ReadWriteCANThread::new(thread_name, thread_requester, thread_receiver))
                });
            }
        }
    }

    pub fn register_ro<F>(&mut self, thread_name: &'static str, thread_func: F)
    where
        F: FnOnce(ReadOnlyCANThread) + std::marker::Send + 'static,
    {
        self.register(thread_name, move |thread_requester, thread_receiver| {
            thread_func(ReadOnlyCANThread::new(thread_name, thread_requester, thread_receiver))
        });
    }

    fn register<F>(&mut self, thread_name: &'static str, func: F)
    where
        F: FnOnce(Sender<ODriveMessage>, Receiver<ODriveResponse>) + std::marker::Send + 'static,
    {
        // Check that the thread ID does not exist already
        if self.threads.contains_key(thread_name) {
            panic!("Two threads cannot have the same id ({})", thread_name);
        }

        // Thread <--- CANManager, sends ODriveResponse
        let (thread_sender, thread_receiver) = channel::<ODriveResponse>();
        let thread_requester = self.mpsc_channel.0.clone();

        // Give the thread the ability to send to the proxy
        // and receive from the proxy
        let thread_handle = std::thread::spawn(move || {
            func(thread_requester, thread_receiver);
        });

        // Add the thread and keep track of it
        self.threads
            .insert(thread_name, (thread_handle, thread_sender));
    }

    pub fn unregister(&mut self, thread_name: &str) -> Result<(), ProxyError> {
        if self.threads.contains_key(thread_name) {
            // unregister from the general thread connections
            let (thread_handle, _sender) = self.threads.remove(thread_name).unwrap();
            match thread_handle.join() {
                Err(_e) => return Err(ProxyError::ThreadFailedJoin),
                _ => {},
            }

            // If the thread being unregistered is the read-write thread, set it to none
            match self.rw_thread {
                Some(rw_thread_name) => {
                    if rw_thread_name == thread_name {
                        self.rw_thread = None
                    }
                },
                _ => {}
            }

            return Ok(())
        } else {
            panic!("Cannot unregister thread ID that doesn't exist")
        }
    }

    /// Take messages off the channel, send it through CAN, and then add it to awaiting
    fn send_to_CAN(&mut self) {
        unimplemented!();

        let receiver = &self.mpsc_channel.1;
        for msg in receiver.recv() {
            self.socket.write_frame(&msg.msg.to_can());
            self.listeners.push(msg);
        }
    }

    /// Receives responses from ODrive
    fn rcv_from_CAN(&self) {
        unimplemented!()
    }

    /// it attempts to match and messages that are waiting to one that was received
    fn match_listener(&self, received_frame: &CANFrame) -> Option<ODriveMessage> {
        unimplemented!()
    }

    /// get the channel for a particular access to respond to
    fn thread_channel(&self, thread_id: usize) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::CANProxy;

    #[test]
    fn test_register_thread() {
        let mut can_proxy = CANProxy::new("fakecan");

        can_proxy.register_ro("thread 1", |read_write_can| {});
        can_proxy.register_ro("thread 2", |read_write_can| {});
        can_proxy.register_rw("thread 3", |read_write_can| {});

        assert_eq!(can_proxy.threads.len(), 3);

        can_proxy.unregister("thread 1");
        assert_eq!(can_proxy.threads.len(), 2);
    }

    #[test]
    #[should_panic]
    fn test_register_duplicate() {
        let mut can_proxy = CANProxy::new("fakecan");

        can_proxy.register_ro("thread 1", |read_write_can| {});
        can_proxy.register_ro("thread 1", |read_write_can| {});
    }

    #[test]
    fn test_unregister_rw_thread() {
        let mut can_proxy = CANProxy::new("fakecan");
        can_proxy.register_rw("thread 1", |read_write_can| {});
        can_proxy.register_ro("thread 2", |read_can| {});

        assert_ne!(can_proxy.rw_thread, None);
        assert_eq!(can_proxy.threads.len(), 2);

        can_proxy.unregister("thread 1");
        can_proxy.unregister("thread 2");

        assert_eq!(can_proxy.rw_thread, None);
        assert_eq!(can_proxy.threads.len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_register_duplicate_rw_thread() {
        let mut can_proxy = CANProxy::new("fakecan");
        can_proxy.register_rw("thread 1", |can_read_write| {});
        can_proxy.register_rw("thread 2", |can_read_write| {});
    }
}
