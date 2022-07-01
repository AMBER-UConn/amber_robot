use std::error::Error;

use rustodrive::{messages::ODriveCANFrame, commands::{Read, ODriveCommand}, canproxy::CANProxy};
use signal_hook::{consts::SIGINT, iterator::Signals};

fn can_testing() -> Result<(), Box<dyn Error>> {
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

    let stop_all = can_proxy.begin();
    
    let mut signals = Signals::new(&[SIGINT])?;    
    for sig in signals.forever() {
        println!("\nQuitting the program {:?}", sig);
        break;
    }

    stop_all().unwrap();
    println!("all done!");
    Ok(())
}

fn main() {
    can_testing();
}