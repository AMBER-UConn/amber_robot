use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

mod can_wrapper;
mod constants; 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


// // Gets the last 
// fn get_axis_id(can_id: &str) -> u32 {
// }

// ///
// fn get_message(can_id: &str) -> u32 {

// }

// struct CANODrive {
//     can_name: &str,
// }

// impl CANODrive{
//     pub fn new(can_name: &str) -> Self {

//     }
//     pub fn send_command(&self, msg: ODriveMessage, recieve: Fn) { unimplemented!() }
//     pub fn 
// }


// fn discover(can_name: &str) {
//     // Look for heartbeats for all axes
//     // let socket = CANSocket::open("can1").expect("Failed to communicate on CAN port");
//     // let odrive_can = CANODrive::new("can1");
//     // let odrives = vec![];

//     let (tx, rx) = mspc::channel();
//     let result = odrive_can.send_blocking(Command { axis: 1, msg: ODrive::SetAxisRequestedState, data: [0, 3, 0, 0, 0, 0, 0, 0] });
//     odrive_can.receive(canid: 0x01, || {

//     });
//     loop {
//         match socket.read_frame() {
//             Ok(response) => {
//                 let axis_id = response.id();
//                 let data = response.data();
//             },
//             Err(error) => {
//                 println!("")
//             }

//         }
//     }
//     // Get latest message from CAN
//     // If it is something that is waiting for a response, take it off the line and 
// }

// struct  CANThread {
//     mapping: Arc<Mutex<HashMap<(id, function)>>>
// }

// impl CANThread {
//     fn begin() -> receiver, sender {
//         std::thread::new(||{
//             loop {
//                 let frame = CANFrame::read();

//                 // on the frame, find the one that matches a command that is waiting
//                 for can_id, on_find in mapping.iter() {
//                     if frame.id() == can_id() {
//                         // call the function with the data that was found
//                         func(Ok(frame.data()))
                            
//                         // remove the function from the hashmap
//                         mapping.remove(id);

//                         // continue the loop
//                         break;
//                     }
//                 }
                
//             }
//         })
//     }
// }