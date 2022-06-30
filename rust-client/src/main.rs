use std::{thread::sleep, time::Duration, sync::atomic::{AtomicBool, Ordering}};

use rustodrive::{canproxy::CANProxy, odrivegroup::ODriveGroup, messages::ODriveCANFrame, commands::{Write, ODriveAxisState, ODriveCommand}};

fn setup_can() {
    let mut can_proxy = CANProxy::new("can1");

    can_proxy.register_rw("thread1", |can_read_write| {
        let mut requests = Vec::new();
        for ax in 0..2 {
            requests.push(ODriveCANFrame {
                axis: ax,
                cmd: ODriveCommand::Write(Write::SetAxisRequestedState),
                data: [ODriveAxisState::FullCalibrationSequence as u8, 0, 0, 0, 0, 0, 0, 0]
            });
        }

        can_read_write.request_many(requests);
        println!("sent calibration sequence command!!");
    });

    let stop_arc = can_proxy.is_alive().to_owned();
    let proxy_handle = std::thread::spawn(move || {can_proxy.begin(); can_proxy});
    
    stop_arc.store(false, Ordering::SeqCst);
    let mut can_proxy = proxy_handle.join().unwrap();
    can_proxy.stop();
    println!("all done!")
}

fn main() {
    setup_can();
}