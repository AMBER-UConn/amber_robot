use std::sync::mpsc::{Receiver, Sender};

use crate::messages::{ODriveMessage, ODriveResponse};

pub trait CANThreadCommunicator {
    /// This passes a message to the CANManager
    fn thread_to_proxy(&self, msg: ODriveMessage) {
        let can_send = self.get_sender();

        // take the message and send it over the channel
        match can_send.send(msg) {
            Ok(()) => {}
            Err(error) => panic!("Lost connection to CANManager thread: \n{}", error),
        }
    }

    /// This waits for a response from the CANManager and returns the result
    ///
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

    /// This returns the send portion of the communication channel to the CANManager
    /// This thread ---> CANManager
    fn get_sender(&self) -> &Sender<ODriveMessage>;

    /// This returns the receive portion of the communication channel from the CANManager
    /// This thread <--- CANManager
    fn get_receiver(&self) -> &Receiver<ODriveResponse>;
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::{channel, Receiver, Sender};

    use crate::{
        commands::{ODriveAxisState, ODriveCommand},
        messages::{ODriveError, ODriveMessage, ODriveResponse},
    };

    use super::CANThreadCommunicator;

    struct ThreadStub {
        thread_receive: Receiver<ODriveResponse>,
        thread_sender: Sender<ODriveMessage>,
        proxy_receive: Receiver<ODriveMessage>,
        proxy_sender: Sender<ODriveResponse>,
    }
    impl ThreadStub {
        fn new() -> Self {
            let (thread_sender, proxy_receive) = channel::<ODriveMessage>();
            let (proxy_sender, thread_receive) = channel::<ODriveResponse>();

            Self {
                thread_receive,
                thread_sender,
                proxy_receive,
                proxy_sender,
            }
        }
    }
    impl CANThreadCommunicator for ThreadStub {
        fn get_receiver(&self) -> &Receiver<ODriveResponse> {
            return &self.thread_receive;
        }
        fn get_sender(&self) -> &Sender<ODriveMessage> {
            return &self.thread_sender;
        }
    }

    #[test]
    fn test_default_thread_to_proxy() {
        let thread = ThreadStub::new();
        let msg = ODriveMessage {
            thread_id: 1,
            axis_id: 1,
            command: ODriveCommand::Heartbeat,
            data: [0, 1, 0, 0, 0, 0, 0, 0],
        };

        thread.thread_to_proxy(msg.clone());

        let data_received = thread.proxy_receive.recv().unwrap();
        assert_eq!(data_received, msg);
    }

    #[test]
    fn test_default_proxy_to_thread() {
        let thread = ThreadStub::new();
        let response = ODriveResponse::Err(ODriveError::FailedToSend);

        thread.proxy_sender.send(response.clone());

        let response_received = thread.proxy_to_thread();
        assert_eq!(response, response_received);
    }
}
