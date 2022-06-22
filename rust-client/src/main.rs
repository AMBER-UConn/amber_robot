use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
    thread::JoinHandle,
};

use rustodrive::{
    canproxy::CANProxy,
    messages::{ODriveMessage, ODriveResponse},
    odrivegroup::ODriveGroup,
};

fn setup_can() {
    let mut can_proxy = CANProxy::new("can1");

    let main_thread = can_proxy.register(|proxy_send, thread_rcv| {
        let odrive = ODriveGroup::new(&[0, 1], proxy_send, thread_rcv);
    });
}

fn main() {
    setup_can();
}
