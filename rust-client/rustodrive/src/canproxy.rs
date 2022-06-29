use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::cansocket::CANSocket;
use crate::messages::{ODriveCANFrame, ODriveError, ODriveMessage, ODriveResponse};
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
    threads_alive: Arc<AtomicBool>,
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
            threads_alive: Arc::new(AtomicBool::new(true)),
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
                let threads_alive_cloned = self.threads_alive.clone();

                self.register(thread_name, move |thread_requester, thread_receiver| {
                    thread_func(ReadWriteCANThread::new(
                        thread_name,
                        thread_requester,
                        thread_receiver,
                        threads_alive_cloned,
                    ))
                });
            }
        }
    }

    pub fn register_ro<F>(&mut self, thread_name: &'static str, thread_func: F)
    where
        F: FnOnce(ReadOnlyCANThread) + std::marker::Send + 'static,
    {
        let threads_alive_cloned = self.threads_alive.clone();

        self.register(thread_name, move |thread_requester, thread_receiver| {
            thread_func(ReadOnlyCANThread::new(
                thread_name,
                thread_requester,
                thread_receiver,
                threads_alive_cloned,
            ))
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
                _ => {}
            }

            // If the thread being unregistered is the read-write thread, set it to none
            match self.rw_thread {
                Some(rw_thread_name) => {
                    if rw_thread_name == thread_name {
                        self.rw_thread = None
                    }
                }
                _ => {}
            }

            return Ok(());
        } else {
            panic!("Cannot unregister thread ID that doesn't exist")
        }
    }

    /// Take messages off the channel, send it through CAN, and then add it to awaiting
    /// TODO add retries for messages that are failing to send
    fn send_queued_msgs(&mut self) {
        let receiver = &self.mpsc_channel.1;

        // This will try to get any values that are available, but otherwise continue on
        // to avoid blocking
        for request in receiver.try_iter() {
            match self.socket.write_frame(&request.msg.to_can()) {
                Ok(_) => self.listeners.push(request),
                Err(_) => self.respond(
                    request.thread_name,
                    ODriveResponse::Err(ODriveError::FailedToSend),
                ),
            }
        }
    }

    /// Keep receiving responses from the odrive unless someone stops the threads
    pub fn begin(&mut self) {
        while self.threads_alive.load(Ordering::SeqCst) {
            self.process_messages();
        }
    }

    pub fn process_messages(&mut self) {
        self.send_queued_msgs();

        // Listen for a response
        let can_response = match self.socket.read_frame() {
            Ok(res) => ODriveCANFrame::from_can(&res),
            Err(_) => return,
        };

        // Find the message that is waiting for a response and send it back
        match self.listener_index(&can_response) {
            Some(index) => {
                let waiting = self.listeners.remove(index);
                self.respond(waiting.thread_name, ODriveResponse::Ok(can_response))
            }
            None => {}
        }
    }

    fn listener_index(&self, received: &ODriveCANFrame) -> Option<usize> {
        self.listeners.iter().position(|msg| msg.msg.is_response(received))
    }

    /// get the channel for a particular access to respond to
    fn respond(&self, thread_name: &'static str, response: ODriveResponse) {
        let (_thread, proxy_responder) = self.threads.get(thread_name).unwrap();
        proxy_responder.send(response).expect(&format!("Proxy cannot reach thread {}", thread_name));
    }

    /// Notify all the threads that they need to stop and then wait for them to do so
    /// Returns thread Error if any single thread fails to stop
    pub fn stop(&mut self) -> std::thread::Result<()> {
        self.threads_alive.store(false, Ordering::SeqCst);

        for thread in self.threads.drain() {
            let (_name, (handle, _sender)) = thread;
            match handle.join() {
                Ok(()) => {},
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use crate::{commands::{Read}, messages::{ODriveCANFrame, ODriveResponse}};

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

    #[test]
    fn test_full_proxy_thread_can_setup() {
        let mut can_proxy = CANProxy::new("fakecan");
        let threads_running = can_proxy.threads_alive.clone();

        // Setup request data
        let mut requests = Vec::new();
        for i in 0..10 {
            requests.push(ODriveCANFrame {
                axis: i,
                cmd: crate::commands::ODriveCommand::Read(Read::EncoderError),
                data: [1; 8],
            })
        }

        // create a channel so that the test thread can communicate back its results
        let (send, rcv) = channel();
        let requests_copy = requests.clone();
        
        // Create thread to send a set of messages and get their responses
        can_proxy.register_rw("thread 1",  move |can_read_write| {
            let responses = can_read_write.request_many(requests_copy);
            send.send(responses).unwrap()
        });


        // We run the proxy on a separate thread to not interfere with checking for responses
        let proxy_thread = std::thread::spawn(move || {
            can_proxy.begin();
            can_proxy
        });
        

        // Keep looping on this thread until it sends a response back through the channel 
        let response: Vec<ODriveResponse>;
        loop {
            match rcv.try_recv() {
                Ok(res) => {
                    response = res;
                    break;
                },
                Err(_) => continue,
            }
        };

        // the mock-socket feature should return the same message sent in as the response
        for resp in response.iter() {
            match resp {
                ODriveResponse::Ok(resp_can_frame) => {
                    assert_eq!(requests.contains(resp_can_frame), true);
                }
                ODriveResponse::Err(_) => {},
            }
        }
            

        // send the signal for all threads to stop
        threads_running.store(false,  std::sync::atomic::Ordering::SeqCst);
        let mut can_proxy = proxy_thread.join().unwrap();
        can_proxy.stop();
    }
}
