use std::{sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{Receiver, Sender},
    Arc,
}, collections::{HashMap, HashSet}};

use crate::{
    commands::{self, ODriveCommand},
    canframe::{ODriveCANFrame, ThreadCANFrame, CANRequest}, response::ODriveResponse,
};

pub(crate) trait CANThreadCommunicator {
    fn new(
        thread_name: &'static str,
        requester: Sender<ThreadCANFrame>,
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

    fn thread_to_proxy(&self, frame: CANRequest) {
        let can_send = self.get_requester();

        // take the message and send it over the channel
        match can_send.send(ThreadCANFrame {
            thread_name: self.thread_name(),
            body: frame,
        }) {
            Ok(()) => {}
            Err(error) => panic!("Lost connection to CANManager thread: \n{}", error),
        }
    }

    /// This returns the Sender portion of the communication channel to the CANManager
    /// This thread ---> CANManager (aka sends requests)
    fn get_requester(&self) -> &Sender<ThreadCANFrame>;

    /// This returns the receive portion of the communication channel from the CANManager
    /// This thread <--- CANManager (aka receives requests)
    fn get_receiver(&self) -> &Receiver<ODriveResponse>;

    /// This sends all the messages specified and waits until responses have been
    /// received for all of them. Responses are returned in the order they were
    /// sent
    fn request_many(&self, requests: Vec<CANRequest>) -> Vec<ODriveResponse> {
        // Check if there are duplicate requests. If there are, panic
        let requests_set = HashSet::<&CANRequest>::from_iter(requests.iter());
        assert!(requests_set.len() == requests.len(), "Duplicate requests contained in call");

        // Send off all the messages
        let num_messages = requests.len();
        for req in requests.iter() {
            self.thread_to_proxy(req.to_owned());
        }

        let mut responses = Vec::new();

        // Wait until you have gotten all of the responses
        while responses.len() < num_messages {
            responses.push(self.proxy_to_thread());
        }

        // Order the responses based on the order they were sent
        let mut responses_map = HashMap::new();
        for resp in responses {
            let assoc_request = match resp.clone() {
                Ok(res_type) => res_type.request(),
                Err(err_type) => err_type.request,
            };
            responses_map.insert(assoc_request, resp);
        }
        let mut ordered_responses = Vec::new();
        for req in requests.iter() {
            ordered_responses.push(responses_map.remove(&req).unwrap());
        }

        ordered_responses
    }

    /// This sends a CANFrame and waits for a response back
    fn request(&self, msg: CANRequest) -> ODriveResponse {
        self.thread_to_proxy(msg);
        self.proxy_to_thread()
    }
}

pub struct ReadWriteCANThread {
    thread_name: &'static str,
    requester: Sender<ThreadCANFrame>,
    receiver: Receiver<ODriveResponse>,
    threads_alive: Arc<AtomicBool>,
}

impl CANThreadCommunicator for ReadWriteCANThread {
    fn new(
        thread_name: &'static str,
        requester: Sender<ThreadCANFrame>,
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

    fn get_requester(&self) -> &Sender<ThreadCANFrame> {
        &self.requester
    }

    fn get_receiver(&self) -> &Receiver<ODriveResponse> {
        &self.receiver
    }
}

impl ReadWriteCANThread {
    pub fn new(
        thread_name: &'static str,
        requester: Sender<ThreadCANFrame>,
        receiver: Receiver<ODriveResponse>,
        threads_alive: Arc<AtomicBool>,
    ) -> Self {
        CANThreadCommunicator::new(thread_name, requester, receiver, threads_alive)
    }
    pub fn request(&self, msg: CANRequest) -> ODriveResponse {
        CANThreadCommunicator::request(self, msg)
    }

    pub fn request_many(&self, messages: Vec<CANRequest>) -> Vec<ODriveResponse> {
        CANThreadCommunicator::request_many(self, messages)
    }

    /// This should look at the shared reference of whether the threads should be running,
    pub fn is_alive(&self) -> bool {
        self.threads_alive.load(Ordering::SeqCst)
    }
}

pub struct ReadOnlyCANThread {
    thread_name: &'static str,
    requester: Sender<ThreadCANFrame>,
    receiver: Receiver<ODriveResponse>,
    threads_alive: Arc<AtomicBool>,
}

impl CANThreadCommunicator for ReadOnlyCANThread {
    fn new(
        thread_name: &'static str,
        requester: Sender<ThreadCANFrame>,
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

    fn get_requester(&self) -> &Sender<ThreadCANFrame> {
        &self.requester
    }

    fn get_receiver(&self) -> &Receiver<ODriveResponse> {
        &self.receiver
    }
}

impl ReadOnlyCANThread {
    pub fn new(
        thread_name: &'static str,
        requester: Sender<ThreadCANFrame>,
        receiver: Receiver<ODriveResponse>,
        threads_alive: Arc<AtomicBool>,
    ) -> Self {
        CANThreadCommunicator::new(thread_name, requester, receiver, threads_alive)
    }
    pub fn request(&self, axis: u32, cmd: commands::ReadComm) -> ODriveResponse {
        CANThreadCommunicator::request(
            self,
            CANRequest {
                axis,
                cmd: commands::ODriveCommand::Read(cmd),
                data: [0; 8],
            },
        )
    }

    pub fn request_many(&self, messages: Vec<(u32, commands::ReadComm)>) -> Vec<ODriveResponse> {
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
    pub fn is_alive(&self) -> bool {
        self.threads_alive.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::{atomic::AtomicBool, Arc}};

    use crate::{
        commands::{ODriveCommand, ReadComm},
        canframe::{ThreadCANFrame, CANRequest},
        tests::ThreadStub,
        threads::CANThreadCommunicator, response::{ErrorResponse, ODriveError},
    };

    #[test]
    fn test_default_read_only_to_proxy() {
        let threads_running = Arc::new(AtomicBool::new(true));
        let thread = ThreadStub::new("test", threads_running);
        

        let can_frame = CANRequest {
            axis: 1,
            cmd: ODriveCommand::Read(ReadComm::Heartbeat),
            data: [0, 0, 0, 0, 0, 0, 0, 0],
        };
        let expected_msg = ThreadCANFrame {
            thread_name: "test",
            body: can_frame,
        };

        thread.rw_communicator.thread_to_proxy(can_frame);

        let data_received = thread.proxy_receiver.recv().unwrap();
        assert_eq!(data_received, expected_msg);
    }

    #[test]
    /// This tests the default implementation in the CANThreadCommuniactor trait
    /// can receive messages using proxy_to_thread()
    fn test_default_proxy_to_thread() {
        let threads_running = Arc::new(AtomicBool::new(true));
        let thread = ThreadStub::new("test", threads_running.clone());

        let fake_request = CANRequest {axis: 1, cmd: ODriveCommand::Read(ReadComm::EncoderError), data: [0;8]};
        let response = Err(ErrorResponse{ request: fake_request, err: ODriveError::FailedToSend});

        thread.proxy_sender.send(response.clone()).unwrap();
        let response_received = thread.rw_communicator.proxy_to_thread();

        assert_eq!(response, response_received);
    }
}
