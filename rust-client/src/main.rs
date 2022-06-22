use rustodrive::{can_manager::{CANManager, ODriveResponse, ODriveMessage}, can_wrapper::ODriveGroup};
use std::{sync::mpsc::{channel, Sender, Receiver}, collections::HashMap};

type AxisSenders = HashMap<usize, Sender<ODriveResponse>>;
type AxisReceivers = HashMap<usize, Receiver<ODriveResponse>>;


const AXIS_IDs: [usize; 5] = [1, 2, 3, 4, 5];

fn gen_thread_comms() -> (AxisSenders, AxisReceivers){

    // Create the sender and receiver vecs. The senders to be given to the CANManager and the receivers to the individual threads
    let (mut thread_senders, mut thread_receivers) = (HashMap::new(), HashMap::new());
    
    for axis in AXIS_IDs.into_iter() {
        let (send, receiver) = channel::<ODriveResponse>();
        thread_senders.insert(axis, send);
        thread_receivers.insert(axis, receiver);
    }
    (thread_senders, thread_receivers)
}


fn setup_can() {
    // Thread <--- CANManager sends ODriveResponse
    let (main_sender, main_receiver) = channel::<ODriveResponse>();

    // Thread(s) --> CANManager (accepts ODriveMessage)
    let (can_manager_send, can_manager_receive) = channel();
        

    let can_man_clone = can_manager_send.clone();
    let main_thread = std::thread::spawn(move || {
        let manager_receiver = main_receiver;
        let manager_sender = can_man_clone;

        let odrive = ODriveGroup::new();
    });

    // let (thread_senders, thread_receivers) = gen_thread_comms(AXIS_IDs);

    
    let can_manager = CANManager::new("can1", can_manager_receive, main_sender);
}


fn main() {
    setup_can();
}