use std::{thread::sleep, time::Duration, sync::atomic::{AtomicBool, Ordering}};
use rustodrive::odrivegroup::test_motor_calib;
use rustodrive::{canproxy::CANProxy, odrivegroup::ODriveGroup, messages::ODriveCANFrame, commands::{Read, ODriveAxisState, ODriveCommand}};

fn setup_can() {
    let mut can_proxy = CANProxy::new("can1");

    can_proxy.register_rw("thread1", |can_read_write| {
        let mut requests = Vec::new();
        for ax in 0..2 {
            requests.push(ODriveCANFrame {
                axis: ax,
                cmd: ODriveCommand::Read(Read::GetVBusVoltage),
                data: [0; 8]
            });
        }

        println!("sent calibration sequence command!!");
        let responses = can_read_write.request_many(requests);
        for res in responses {
            println!("response: {:?}", res);
        }
    });
    let stop_arc = can_proxy.is_alive().to_owned();
    let proxy_handle = std::thread::spawn(move || {can_proxy.begin(); can_proxy});


    std::thread::sleep(Duration::new(10, 0));
    
    stop_arc.store(false, Ordering::SeqCst);
    let mut can_proxy = proxy_handle.join().unwrap();
    can_proxy.stop();
    println!("all done!")
}

fn main() {
    setup_can();
    // test_motor_calib();
}