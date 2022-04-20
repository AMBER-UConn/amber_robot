use crate::keyboard::KeyboardSender;
use rosrust;
pub mod gamepad;
pub mod keyboard;

mod msg {
    rosrust::rosmsg_include!(std_msgs/String);
}

fn main() {
    // Initialize node

    // Create object that maintains 10Hz between sleep requests
    let mut kb_send = KeyboardSender::new("kbstream".to_string());
    kb_send.start_kb_stream();
}