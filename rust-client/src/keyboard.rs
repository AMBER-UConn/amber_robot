use std::io::Read;
use std::io::{Write, Stdin};
use std::iter::{Iterator};
use rosrust;
use termios::tcsetattr;
use termios::Termios;
use std::collections::HashMap;
use crate::gamepad::{Button};
use crate::gamepad::{GamepadReciever, GamepadSender};
use std::sync::{
    Arc,
    atomic::AtomicBool
};

pub struct KeyboardRecieve {
    button_map: HashMap<char, Button>, 
    topic_name: String,
}

impl KeyboardRecieve {
    fn new(topic: String, button_map: HashMap<char, Button>) -> KeyboardRecieve {
        return KeyboardRecieve{
            topic_name: topic, 
            button_map: button_map, 
        };
    }
}

impl GamepadReciever for KeyboardRecieve {
    fn get_message(&self) {
        
    }

    fn translate(&self, keypress: &char) -> Button {
        match self.button_map.get(keypress) {
            Some(button) => return button.clone(),
            None => return Button::UNDEFINED,
        }
    }
    
    fn has_message(&self) {
        // TODO write code here
    }
}


pub struct KeyboardSender {
    kb_stream_on: Arc<AtomicBool>,
    topic_name: String
}

impl KeyboardSender {

    pub fn new(topic_name: String) -> KeyboardSender {
        return KeyboardSender {
            kb_stream_on: Arc::new(AtomicBool::new(false)),
            topic_name 
        };
    }

    pub fn send_message(&self, msg: &str) {
        
    }

    pub fn start_kb_stream(&mut self) {
        // https://github.com/openrr/openrr/blob/main/arci-gamepad-keyboard/src/lib.rs

        // WIP (work in progress)
        // Based on https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
        let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work on /dev/stdin or /dev/tty
        let termios = Termios::from_fd(stdin).unwrap();
        let mut new_termios = termios; // make a mutable copy of termios to modify

        new_termios.c_lflag &= !(termios::ICANON | termios::ECHO); // no echo and no canonical mode
        tcsetattr(stdin, termios::TCSANOW, &new_termios).unwrap(); 

        // let stdout = std::io::stdout();
        let mut reader = std::io::stdin();
        // stdout.lock().flush().unwrap();
        // drop(stdout);

        self.kb_stream_on = Arc::new(AtomicBool::new(true));
        let topic_name = self.topic_name.clone();

        // std::thread::spawn(move || {
        rosrust::init(topic_name.as_str());
        let chatter_pub = rosrust::publish(topic_name.as_str(), 100).unwrap();

        let rate = rosrust::rate(10.0);

        while rosrust::is_ok() {
            let mut keybuff = [0; 1]; // Read one key at a time
            reader.read(&mut keybuff).unwrap();
            let key_pressed = keybuff[0];
            
            let mut msg = rosrust_msg::std_msgs::String::default();
            msg.data = format!("Keys pressed: {:?}", &key_pressed );
            chatter_pub.send(msg).unwrap();
            rate.sleep();
        }
        println!("ROS IS NOT OKAY IT IS HURT!");
        tcsetattr(stdin, termios::TCSANOW, &termios).unwrap(); // reset the stdin to

        // });
    }
}

impl GamepadSender for KeyboardSender {
    fn send_message(&self) {

    }
} 


        
// loop {
//     let recieved_data = await kb.recieve();
//     let buttons_pressed = kb.convert_chars(recieved_data);
//     kb.update_state(buttons_pressed);

//     for button in buttons_pressed {
//         match button {
//             Button::BACKWARD => {println!();}
//         }
//     }
// }



/// ___________RECIEVER NODE______________
/// control_robot_node.rs --------
/// 
/// let kb_recieve = KBReciever("/keyboard_input");
/// 
/// while kb_recieve.has_message() {
///     let characters = kb_recieve.get_message(); <--- an array of chars ['w', 'a', 's', 'd']
///     
///     for char_pressed in characters {
///         let button_pressed = kb_recieve.translate(char_pressed); <--- input ('w'): outputs Button::Forward
///     
///         match button_pressed {
    ///         Button::Forward => do_something();
    ///         Button::Backward => yell();
    ///         Button::Left => jump();
    ///         etc.... 
///         }
///     }
///     
/// }
/// 
/// _________PUBLISH NODE_____________
/// 
/// let kb_send = KBSender("/keyboard_input")
/// 
/// fn read_kb_input() {
///     /* *** */
/// }
/// fn has_kb_input() {
///     /* *** */
/// }
/// while has_kb_input() {
///     let actual_kb_input = read_kb_input();
///     kb_send.send_message(actual_kb_input)
/// }
/// 
/// 



pub fn default_buttons()->HashMap<char, Button>{
    let mut buttons = HashMap::new();
    buttons.insert('w', Button::FORWARD);
    buttons.insert('a', Button::LEFT);
    buttons.insert('s', Button::BACKWARD);
    buttons.insert('d', Button::RIGHT);
    buttons.insert('<', Button::CCW);
    buttons.insert('>', Button::CW);
    buttons.insert('t', Button::MODE_SHIFT);
    
    return buttons;
}

#[cfg(test)]
mod tests{
    use crate::gamepad::GamepadReciever;
    use crate::keyboard::{KeyboardRecieve, default_buttons};
    use crate::gamepad::Button;

    #[test]
    fn test_button_conversion(){
        let kb = KeyboardRecieve::new(String::from("/keyboard_input"), default_buttons());
        assert_eq!(Button::FORWARD, kb.translate(&'w'));
        assert_eq!(Button::UNDEFINED, kb.translate(&'%'));
    }
}