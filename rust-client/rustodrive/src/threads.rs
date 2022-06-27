use std::sync::mpsc::{Receiver, Sender};

use crate::{
    commands::{self},
    messages::{ODriveCANFrame, ODriveMessage, ODriveResponse},
};

pub(crate) trait CANThreadCommunicator {
    fn new(
        thread_name: &'static str,
        requester: Sender<ODriveMessage>,
        receiver: Receiver<ODriveResponse>,
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
            Err(error) => panic!("Lost connection to CANManager thread: \n{}", error),
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
    fn request_many(&self, messages: Vec<(u32, ODriveCANFrame)>) -> Vec<ODriveResponse> {
        // Send off all the messages
        let num_messages = messages.len();
        for (axis, msg) in messages {
            self.thread_to_proxy(msg);
        }

        let mut responses = Vec::new();

        // Wait until you have gotten all of the responses
        while responses.len() < num_messages {
            match self.get_receiver().recv() {
                Ok(res) => responses.push(res),
                Err(err) => panic!("Thread {} disconnected: \n{}", self.thread_name(), err),
            }
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
}

impl CANThreadCommunicator for ReadWriteCANThread {
    fn new(
        thread_name: &'static str,
        requester: Sender<ODriveMessage>,
        receiver: Receiver<ODriveResponse>,
    ) -> Self {
        Self {
            thread_name,
            requester,
            receiver,
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
    ) -> Self {
        CANThreadCommunicator::new(thread_name, requester, receiver)
    }
    pub fn request(&self, msg: ODriveCANFrame) -> ODriveResponse {
        CANThreadCommunicator::request(self, msg)
    }

    pub fn request_many(&self, messages: Vec<(u32, ODriveCANFrame)>) -> Vec<ODriveResponse> {
        CANThreadCommunicator::request_many(self, messages)
    }
}

pub struct ReadOnlyCANThread {
    thread_name: &'static str,
    requester: Sender<ODriveMessage>,
    receiver: Receiver<ODriveResponse>,
}

impl CANThreadCommunicator for ReadOnlyCANThread {
    fn new(
        thread_name: &'static str,
        requester: Sender<ODriveMessage>,
        receiver: Receiver<ODriveResponse>,
    ) -> Self {
        Self {
            thread_name,
            requester,
            receiver,
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
    ) -> Self {
        CANThreadCommunicator::new(thread_name, requester, receiver)
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

    pub fn request_many(&self, messages: Vec<(u32, ODriveCANFrame)>) -> Vec<ODriveResponse> {
        CANThreadCommunicator::request_many(self, messages)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::{ODriveCommand, Read},
        messages::{ODriveCANFrame, ODriveError, ODriveMessage, ODriveResponse},
        tests::ThreadStub,
        threads::CANThreadCommunicator,
    };

    #[test]
    fn test_default_read_only_to_proxy() {
        let thread = ThreadStub::new("test");
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
        let thread = ThreadStub::new("test");
        let response = ODriveResponse::Err(ODriveError::FailedToSend);

        thread.proxy_sender.send(response.clone());
        let response_received = thread.rw_communicator.proxy_to_thread();

        assert_eq!(response, response_received);
    }
}
