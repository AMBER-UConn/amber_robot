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
