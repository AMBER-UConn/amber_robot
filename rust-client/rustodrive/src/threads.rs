use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{Receiver, Sender},
    Arc,
};

use crate::{
    commands::{self, ODriveCommand},
    messages::{ODriveCANFrame, ODriveMessage, ODriveResponse},
};

pub(crate) trait CANThreadCommunicator {
    fn new(
        thread_name: &'static str,
        requester: Sender<ODriveMessage>,
        receiver: Receiver<ODriveResponse>,
        threads_alive: Arc<AtomicBool>,
    ) -> Self;
    fn thread_name(&self) -> &'static str;

    /// This waits for a response from the CANManager and returns the result
    /// This is responsible for returning any errors related to invalid commands
    /// and or any errors that occur during the odrive's command execution
    fn proxy_to_thread(&self) -> ODriveResponse {
        // wait for the response from the thread and return
        let can_recv = self.get_receiver();

        match can_recv.recv() {
            Ok(response) => return response,
            Err(error) => panic!("Thread {} disconnected: \n{}", self.thread_name(), error),
        }
    }

    fn thread_to_proxy(&self, frame: ODriveCANFrame) {
        let can_send = self.get_requester();

        // take the message and send it over the channel
        match can_send.send(ODriveMessage {
            thread_name: self.thread_name(),
            msg: frame,
        }) {
            Ok(()) => {}
            Err(error) => panic!("Lost connection to CANManager thread: \n{}", error),
        }
    }

    /// This returns the Sender portion of the communication channel to the CANManager
    /// This thread ---> CANManager (aka sends requests)
    fn get_requester(&self) -> &Sender<ODriveMessage>;

    /// This returns the receive portion of the communication channel from the CANManager
    /// This thread <--- CANManager (aka receives requests)
    fn get_receiver(&self) -> &Receiver<ODriveResponse>;

    /// This sends all the messages specified and waits until responses have been
    /// received for all of them
    fn request_many(&self, requests: Vec<ODriveCANFrame>) -> Vec<ODriveResponse> {
        // Send off all the messages
        let num_messages = requests.len();
        for req in requests {
            self.thread_to_proxy(req);
        }

        let mut responses = Vec::new();

        // Wait until you have gotten all of the responses
        while responses.len() < num_messages {
            responses.push(self.proxy_to_thread());
        }

        responses
    }

    /// This sends a CANFrame and waits for a response back
    fn request(&self, msg: ODriveCANFrame) -> ODriveResponse {
        self.thread_to_proxy(msg);
        self.proxy_to_thread()
    }
}

pub struct ReadWriteCANThread {
    thread_name: &'static str,
    requester: Sender<ODriveMessage>,
    receiver: Receiver<ODriveResponse>,
    threads_alive: Arc<AtomicBool>,
}

impl CANThreadCommunicator for ReadWriteCANThread {
    fn new(
        thread_name: &'static str,
        requester: Sender<ODriveMessage>,
        receiver: Receiver<ODriveResponse>,
        threads_alive: Arc<AtomicBool>,
    ) -> Self {
        Self {
            thread_name,
            requester,
            receiver,
            threads_alive,
        }
    }

    fn thread_name(&self) -> &'static str {
        self.thread_name
    }

    fn get_requester(&self) -> &Sender<ODriveMessage> {
        &self.requester
    }

    fn get_receiver(&self) -> &Receiver<ODriveResponse> {
        &self.receiver
    }
}

impl ReadWriteCANThread {
    pub fn new(
        thread_name: &'static str,
        requester: Sender<ODriveMessage>,
        receiver: Receiver<ODriveResponse>,
        threads_alive: Arc<AtomicBool>,
    ) -> Self {
        CANThreadCommunicator::new(thread_name, requester, receiver, threads_alive)
    }
    pub fn request(&self, msg: ODriveCANFrame) -> ODriveResponse {
        CANThreadCommunicator::request(self, msg)
    }

    /// Takes
    pub fn request_many(&self, messages: Vec<ODriveCANFrame>) -> Vec<ODriveResponse> {
        CANThreadCommunicator::request_many(self, messages)
    }

    /// This should look at the shared reference of whether the threads should be running,
    pub fn check_alive(&self) -> bool {
        self.threads_alive.load(Ordering::SeqCst)
    }
}

pub struct ReadOnlyCANThread {
    thread_name: &'static str,
    requester: Sender<ODriveMessage>,
    receiver: Receiver<ODriveResponse>,
    threads_alive: Arc<AtomicBool>,
}

impl CANThreadCommunicator for ReadOnlyCANThread {
    fn new(
        thread_name: &'static str,
        requester: Sender<ODriveMessage>,
        receiver: Receiver<ODriveResponse>,
        threads_alive: Arc<AtomicBool>,
    ) -> Self {
        Self {
            thread_name,
            requester,
            receiver,
            threads_alive,
        }
    }

    fn thread_name(&self) -> &'static str {
        self.thread_name
    }

    fn get_requester(&self) -> &Sender<ODriveMessage> {
        &self.requester
    }

    fn get_receiver(&self) -> &Receiver<ODriveResponse> {
        &self.receiver
    }
}

impl ReadOnlyCANThread {
    pub fn new(
        thread_name: &'static str,
        requester: Sender<ODriveMessage>,
        receiver: Receiver<ODriveResponse>,
        threads_alive: Arc<AtomicBool>,
    ) -> Self {
        CANThreadCommunicator::new(thread_name, requester, receiver, threads_alive)
    }
    pub fn request(&self, axis: u32, cmd: commands::Read) -> ODriveResponse {
        CANThreadCommunicator::request(
            self,
            ODriveCANFrame {
                axis,
                cmd: commands::ODriveCommand::Read(cmd),
                data: [0; 8],
            },
        )
    }

    pub fn request_many(&self, messages: Vec<(u32, commands::Read)>) -> Vec<ODriveResponse> {
        let requests = messages
            .iter()
            .map(|(axis, cmd)| ODriveCANFrame {
                axis: *axis,
                cmd: ODriveCommand::Read(*cmd),
                data: [0; 8],
            })
            .collect();
        CANThreadCommunicator::request_many(self, requests)
    }

    /// This should look at the mutex of whether the threads should be running,
    pub fn check_alive(&self) -> bool {
        self.threads_alive.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{atomic::AtomicBool, Arc};

    use crate::{
        commands::{ODriveCommand, Read},
        messages::{ODriveCANFrame, ODriveError, ODriveMessage, ODriveResponse},
        tests::ThreadStub,
        threads::CANThreadCommunicator,
    };

    #[test]
    fn test_default_read_only_to_proxy() {
        let threads_running = Arc::new(AtomicBool::new(true));
        let thread = ThreadStub::new("test", threads_running);
        

        let can_frame = ODriveCANFrame {
            axis: 1,
            cmd: ODriveCommand::Read(Read::Heartbeat),
            data: [0, 0, 0, 0, 0, 0, 0, 0],
        };
        let expected_msg = ODriveMessage {
            thread_name: "test",
            msg: can_frame,
        };

        thread.rw_communicator.thread_to_proxy(can_frame);

        let data_received = thread.proxy_receiver.recv().unwrap();
        assert_eq!(data_received, expected_msg);
    }

    #[test]
    fn test_default_proxy_to_thread() {
        let threads_running = Arc::new(AtomicBool::new(true));
        let thread = ThreadStub::new("test", threads_running.clone());

        let response = ODriveResponse::Err(ODriveError::FailedToSend);

        thread.proxy_sender.send(response.clone());
        let response_received = thread.rw_communicator.proxy_to_thread();

        assert_eq!(response, response_received);
    }
}
