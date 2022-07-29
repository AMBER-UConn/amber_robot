use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::cansocket::CANSocket;
use crate::commands::ODriveCommand;
use crate::messages::{CANResponse, ODriveMessage, ODriveCANFrame};
use crate::response::{ODriveResponse, ResponseType, ErrorResponse, ODriveError};
use crate::threads::{ReadOnlyCANThread, ReadWriteCANThread};

type ThreadConnection = (JoinHandle<()>, Sender<ODriveResponse>);
type ThreadID = &'static str;
pub enum ProxyError {
    ThreadFailedJoin,
    ThreadDuplicateID,
}

/// The CANProxy is in charge of handling all communication with the CAN
/// port on behalf of all threads that are registered to it.
pub struct CANProxy {
    mpsc_channel: (Sender<ODriveMessage>, Receiver<ODriveMessage>),
    threads: HashMap<ThreadID, ThreadConnection>,
    rw_thread: Option<ThreadID>, // There can only be one read and write thread at a time. Store the identifier in here
    threads_alive: Arc<AtomicBool>,
    requests: Vec<ODriveMessage>,
    socket: CANSocket,
}

impl CANProxy {
    /// Instantiates a new CANProxy. Only one CANProxy should be instantiated at a time
    /// # Arguments
    /// * `can_device` - a string slice to the CAN port name
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
            requests: vec![],
            threads_alive: Arc::new(AtomicBool::new(true)),
        }
    }

    /// This registers a new thread that is given a handle with read and write
    /// access to CAN (in this case [`ReadWriteCANThread`])
    ///
    /// Only one thread can have read and write access. This function will
    /// panic if multiple threads are registered as read and write.
    ///
    /// # Arguments
    /// * `thread_name` - a unique identifier to refer to that thread you registered
    /// so that you can unregister it. Registering panics if a duplicate name exists
    /// * `thread_func` - this is a closure that is the entry point to code execution
    /// in the separate thread. What is contained must implement the `Send` trait.
    ///
    /// The closure takes one argument which is a [`ReadWriteCANThread`] object
    ///
    /// # Example
    /// ```
    /// use rustodrive::canproxy::CANProxy;
    /// use rustodrive::messages::CANRequest;
    /// use rustodrive::commands::{ODriveCommand::Read, ReadComm};
    ///
    /// let mut can_proxy = CANProxy::new("can0");
    /// can_proxy.register_rw("thread 1", |can_read_write| {
    ///     // .request() blocks until a response is received
    ///     can_read_write.request(CANRequest {
    ///         axis: 1,
    ///         cmd: Read(ReadComm::GetVBusVoltage),
    ///         data: [0; 8]
    ///     });
    /// });
    ///
    /// // start processing of messages on a separate thread
    /// let stop_threads = can_proxy.begin();
    /// std::thread::sleep_ms(1000);
    ///
    /// // Send the signal for all registered threads to stop
    /// // and wait for them to join with the hook given by .begin()
    /// stop_threads().unwrap();
    /// ```
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

    /// This registers a new thread that is given a handle with read only
    /// access to CAN (in this case [`ReadOnlyCANThread`])
    ///
    /// An unlimited number of read-only threads can be registered.
    ///
    /// # Arguments
    /// * `thread_name` - a unique identifier to refer to that thread you registered
    /// so that you can unregister it. Registering panics if a duplicate name exists
    /// * `thread_func` - this is a closure that is the entry point to code execution
    /// in the separate thread. What is contained must implement the `Send` trait.
    ///
    /// The closure takes one argument which is a [`ReadOnlyCANThread`] object
    /// # Example
    /// ```
    /// use rustodrive::canproxy::CANProxy;
    /// use rustodrive::commands::ReadComm;
    ///
    /// let mut can_proxy = CANProxy::new("can0");
    /// can_proxy.register_ro("thread 1", |can_read| {
    ///     // .request() blocks until a response is received
    ///     let axis = 1;
    ///     let cmd = ReadComm::GetVBusVoltage;    
    ///     can_read.request(axis, cmd);
    /// });
    ///
    /// // start processing of messages on a separate thread
    /// let stop_threads = can_proxy.begin();
    /// std::thread::sleep_ms(1000);
    ///
    /// // Send the signal for all registered threads to stop
    /// // and wait for them to join with the hook given by .begin()
    /// stop_threads().unwrap();
    /// ```
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

    /// This is a helper function that does the bulk of the work to instantiate a
    /// new thread. This checks if duplicate thread_names are used.
    ///
    /// This thread initializes the communication channels between the CANProxy and the
    /// [`ReadOnlyCANThread`]/[`ReadWriteCANThread`].
    ///
    /// Additionally, the register function stores the join handle for the thread
    /// for joining all the threads at a later point.
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
            return func(thread_requester, thread_receiver);
        });

        // Add the thread and keep track of it
        self.threads
            .insert(thread_name, (thread_handle, thread_sender));
    }

    /// This unregisters a thread with the given identifier. If the thread fails
    /// to `.join()`, the result gets propogated and the remaining threads
    /// are not joined. 
    /// 
    /// This function panics if a thread that is not registered is unregistered.
    ///
    /// ### Example 1
    /// ```
    /// use rustodrive::canproxy::CANProxy;
    ///
    /// let mut can_proxy = CANProxy::new("can0");
    /// can_proxy.register_ro("thread 1", |can_read| {});
    /// 
    /// can_proxy.unregister("thread 1").expect("thread 1 could did not join");
    /// ```
    /// 
    /// ### Example 2
    /// The thread being registered may need to loop continuously. To handle the "stop"
    /// signal, the provided [`ReadOnlyCANThread`]/[`ReadWriteCANThread`] object has a method
    /// `.is_alive()` that can be used as so to peacefully exit the thread:
    /// ```
    /// use rustodrive::canproxy::CANProxy;
    ///
    /// let mut can_proxy = CANProxy::new("can0");
    /// can_proxy.register_ro("thread 1", |can_read| {
    ///     while can_read.is_alive() {
    ///         // do stuff
    ///     }
    ///     println!("Exit handled!");
    /// });
    /// 
    /// can_proxy.stop_threads();
    /// can_proxy.unregister("thread 1").expect("thread 1 could did not join");
    /// ``` 
    /// ## Important note
    /// - Joining the thread does not return anything. This is because
    /// a closure could potentially return anything. To rectify this, it would require
    /// the use of generics and possibly `dyn Box` but currently this is not supported. 
    pub fn unregister(&mut self, thread_name: &str) -> std::thread::Result<()> {
        if self.threads.contains_key(thread_name) {
            // unregister from the general thread connections
            let (thread_handle, _sender) = self.threads.remove(thread_name).unwrap();
            match thread_handle.join() {
                Err(e) => return Err(e),
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

    /// This functions takes any CAN messages that were sent by various
    /// registered threads and then sends them off to the CAN bus. 
    /// 
    /// If it is a `commands::Read` command, then we save it for later and wait for a response.
    /// If it is a `commands::Write` command, then we can respond with `ODriveResponse::ReqReceived`
    /// as soon as the CAN bus accepts the message without error.
    /// 
    /// If there is an error in connecting to the CAN bus, this immediately
    /// responds back to the thread that sent the blocking request
    /// that there was an error.
    // TODO add retries for messages that are failing to send
    fn send_queued_msgs(&mut self) {
        let receiver = &self.mpsc_channel.1;

        // This will try to get any messages that are available to send, otherwise the method
        // returns if there is nothing available to avoid blocking
        for request in receiver.try_iter() {
            // If the command is a read, it must have the RTR bit enabled
            // since it is waiting for a response
            let rtr_enabled = match request.body.cmd {
                ODriveCommand::Read(_) => true,
                ODriveCommand::Write(_) => false,
            };

            match self.socket.write_frame(&request.body.to_can(rtr_enabled)) {
                Ok(_) => {
                    match request.body.cmd {
                        // If the request was successfully sent and it is a Write request, notify that it was sucessfully sent
                        ODriveCommand::Write(_) => {
                            self.respond(request.thread_name, Ok(ResponseType::Bodyless { req: request.body }));
                        }
                        // otherwise add the message as a listener
                        ODriveCommand::Read(_) => self.requests.push(request),
                    }
                }
                // If there was an error with writing the frame, respond back with the
                // the attempted request and the error
                Err(_) => self.respond(
                    request.thread_name,
                    Err(ErrorResponse{ request: request.body, err: ODriveError::FailedToSend}),
                ),
            }
        }
    }

    /// This function reads from the CAN socket. If there is a `commands::Write`
    /// request waiting for a response from the CAN bus, this function
    /// will respond to the appropriate thread with a [`ODriveResponse`]
    /// containing the data of the response. 
    fn handle_can_response(&mut self) {
        // Listen for a response
        let can_response = match self.socket.read_frame() {
            Ok(res) => CANResponse::from_can(&res),
            Err(_) => return,
        };
        //print!("{:?}", &can_response);

        // Find the message that is waiting for a response and send it back
        match self.listener_index(&can_response) {
            Some(index) => {
                // println!("response matched with smth from odrive {:?}", can_response);

                let waiting = self.requests.remove(index);
                self.respond(
                    waiting.thread_name,
                    Ok(ResponseType::Body {request: waiting.body, response: can_response}),
                )
            }
            None => {}
        }
    }

    /// This function consumes `self` and starts a separate thread that constantly
    /// processes any messages that are received by threads. This thread responds
    /// to the same stop signal as all other threads.
    /// 
    /// This returns a function/hook that is capable of stopping all threads,
    /// including the thread where CANProxy is running. This is necessary because
    /// the CANProxy thread is not tracked in the same manner as registered threads.
    /// 
    /// The resulting hook returns the CANProxy object if all the threads were
    /// sucessfully joined
    /// 
    /// ## Example
    /// ```
    /// use rustodrive::canproxy::CANProxy;
    ///
    /// let mut can_proxy = CANProxy::new("can0");
    /// can_proxy.register_ro("thread 1", |can_read| {});
    ///
    /// // start processing of messages on a separate thread
    /// let stop_threads = can_proxy.begin(); // <--- can_proxy is consumed here
    /// std::thread::sleep_ms(1000);
    ///
    /// // Send the signal for all registered threads to stop
    /// // and wait for them to join with the hook given by .begin()
    /// let can_proxy = stop_threads().unwrap(); // <--- this is the same can_proxy object as before
    /// ```
    pub fn begin(mut self) -> impl FnOnce() -> std::thread::Result<CANProxy> {
        let threads_alive_copy = self.threads_alive.clone();

        let proxy_handle = std::thread::spawn(move || {
            while self.is_alive() {
                self.process_messages();
            }
            return self;
        });

        // Send the signal for the proxy_handle and all threads to finish up their work
        let stop_all = move || {
            threads_alive_copy.store(false, Ordering::SeqCst);

            // wait for proxy thread to finish
            let mut proxy = match proxy_handle.join() {
                Ok(p) => p,
                Err(err) => return Err(err),
            };

            // Then stop and wait for all the threads that were registered
            match proxy.join_registered() {
                Ok(()) => return Ok(proxy),
                Err(e) => return Err(e),
            }
        };

        return stop_all;
    }

    /// This function updates a shared reference that all threads have access to
    /// to notify them that they should stop execution.
    pub fn stop_threads(&self) {
        self.threads_alive.store(false, Ordering::SeqCst);
    }

    /// This function handles the processing of messages from registered threads
    /// and responding to threads as soon as their request has been fulfilled.
    /// 
    /// This thread most likely needs to continuously loop in a separate thread.
    /// This functionality has been gratiously implemented with [`CANProxy::begin()`]
    pub fn process_messages(&mut self) {
        self.send_queued_msgs();
        self.handle_can_response();
    }

    /// This function returns the index of the stored request waiting
    /// for a response
    fn listener_index(&self, received: &ODriveCANFrame) -> Option<usize> {
        self.requests
            .iter()
            .position(|msg| msg.body.is_response(received))
    }

    /// This finds the thread based on the identifier and sends the specified
    /// [`ODriveResponse`] across the response channel for the thread
    fn respond(&self, thread_name: &'static str, response: ODriveResponse) {
        let (_thread, proxy_responder) = self.threads.get(thread_name).expect(&format!("The thread {} was not registered and cannot be responded to", thread_name));
        proxy_responder
            .send(response)
            .expect(&format!("Proxy cannot reach thread {}", thread_name));
    }

    /// This returns whether or not all threads are running. By default
    /// this is true, unless all threads have been specifically stopped.
    /// 
    /// Once all threads have been stopped, they cannot be restarted.
    pub fn is_alive(&self) -> bool {
        return self.threads_alive.load(Ordering::SeqCst);
    }

    /// This waits for all threads to join. If a single one fails to join,
    /// that result is propogated upwards. 
    /// 
    /// This method does not send the stop signal to the threads, so you 
    /// may have to call `.stop_threads()` or use the hook from `.begin()`.
    /// 
    /// ## Example
    /// ```
    /// use rustodrive::canproxy::CANProxy;
    ///
    /// let mut can_proxy = CANProxy::new("can0");
    /// can_proxy.register_ro("thread 1", |can_read| {});
    /// std::thread::sleep_ms(1000);
    /// 
    /// can_proxy.stop_threads();
    /// can_proxy.join_registered();
    /// ```
    pub fn join_registered(&mut self) -> std::thread::Result<()> {
        let thread_names: Vec<&str> = self.threads.iter().map(|(name, _else)| *name).collect();
        for name in thread_names {
            match self.unregister(name) {
                Ok(()) => {},
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc::channel, time::Duration};

    use crate::{
        commands::{ODriveCommand, ReadComm, WriteComm},
        messages::{CANRequest}, tests::wait_for_msgs, response::{ManyResponses, ODriveResponse, ResponseType},
    };

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

        // can_proxy.unregister("thread 1");
        // can_proxy.unregister("thread 2");

        can_proxy.stop_threads();
        can_proxy.join_registered();

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
    /// Test that a Read command responds back with a response (should be the same one for testing purposes)
    fn test_read_command_response() {
        let mut can_proxy = CANProxy::new("fakecan");
        let request = CANRequest {
            axis: 2,
            cmd: ODriveCommand::Read(ReadComm::EncoderError),
            data: [0; 8],
        };

        // create a channel so that the test thread can communicate back its results
        let (send, rcv) = channel();
        let request_copy = request.clone();

        // Create thread to send a set of messages and get their responses
        can_proxy.register_rw("thread 1", move |can_read_write| {
            let response = can_read_write.request(request_copy);
            send.send(response).unwrap()
        });

        let stop_proxy = can_proxy.begin();

        // Keep looping on this thread until it sends a response back through the channel
        let response = wait_for_msgs(rcv);
        stop_proxy().unwrap();

        // Assert the response body is the same as the CANFrame that was sent in the request
        // because it was a read request using mock-socket
        let can_response = response.unwrap().body().1;
        assert_eq!(request.axis, can_response.axis);
        assert_eq!(request.cmd, can_response.cmd);
        assert_ne!(request.data, can_response.data);

    }

    #[test]
    /// Write command responds with MsgReceived to notify it was sent over the CAN bus
    fn test_write_command_response() {
        let mut can_proxy = CANProxy::new("fakecan");
        let request = CANRequest {
            axis: 2,
            cmd: ODriveCommand::Write(WriteComm::SetAxisNodeID),
            data: [0; 8],
        };

        // create a channel so that the test thread can communicate back its results
        let (send, rcv) = channel();
        let request_copy = request.clone();

        // Create thread to send a set of messages and get their responses
        can_proxy.register_rw("thread 1", move |can_read_write| {
            let response = can_read_write.request(request_copy);
            send.send(response).unwrap()
        });

        let stop_proxy = can_proxy.begin();

        // Keep looping on this thread until it sends a response back through the channel
        let response = wait_for_msgs(rcv);
        stop_proxy().unwrap();

        // Assert the response body is the same as the CANFrame that was sent in the request
        // because it was a read request using mock-socket
        assert_eq!(response, Ok(ResponseType::Bodyless{req: request}));

    }

    #[test]
    /// Testing that the response is sorted according to the order it was sent
    /// This is because we cannot assume the odrive will respond sequentially
    /// to requests.
    fn test_request_many() {
        let mut can_proxy = CANProxy::new("fakecan");

        // Setup request data
        let mut requests = Vec::new();
        for i in 0..10 {
            requests.push(CANRequest {
                axis: i,
                cmd: ODriveCommand::Read(ReadComm::EncoderError),
                data: [1; 8],
            })
        }

        // create a channel so that the test thread can communicate back its results
        let (send, rcv) = channel();
        let requests_copy = requests.clone();

        // Create thread to send a set of messages and get their responses
        can_proxy.register_rw("thread 1", move |can_read_write| {
            let responses = can_read_write.request_many(requests_copy);
            send.send(responses).unwrap()
        });

        // we sleep for a short amount of time so that that messages can build up and be
        // randomly returned to test that the response is sorted properly
        std::thread::sleep(Duration::from_millis(500));

        let stop_all = can_proxy.begin();
        let response = wait_for_msgs(rcv).unwrap_all();
        stop_all().unwrap();

        // The response should be returned in the same order as the requests made
        for (expected_req, res) in requests.into_iter().zip(response) {
            let (_, actual_response) = res.body();
            assert_eq!(actual_response.is_response(&expected_req), true);
        }
    }
}
