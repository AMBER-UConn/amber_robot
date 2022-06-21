use rustodrive::can_manager::{CANManager, ODriveResponse};
use std::{sync::mpsc::channel, collections::HashMap};


fn setup_can() {
    let odrive_axes: Vec<usize> = vec![1, 2, 3, 4, 5];

    // Create the sender and receiver vecs. The senders to be given to the CANManager and the receivers to the individual threads
    let (mut thread_senders, mut thread_receivers) = (HashMap::new(), HashMap::new());
    for axis in odrive_axes.into_iter() {
        let (send, receiver) = channel::<ODriveResponse>();
        thread_senders.insert(axis, send);
        thread_receivers.insert(axis, receiver);
    }

    let (can_manager_send, can_manager_receive) = channel();
    let can_manager = CANManager::new(can_manager_receive, thread_senders);
}


fn main() {
    setup_can();
}