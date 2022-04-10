use crate::gamepad::{Button, Gamepad};


pub struct keyboard{
    button_map: HashMap<char, Button>, 
    state: HashMap<char, bool>,
}

// 
impl Gamepad for keyboard{
    
    fn send(&self){
        #todo
    }

    fn recieve(&self){
        #todo
    }
}

pub default_buttons()->HashMap<char, Button>{
    let buttons = HashMap::new();
    buttons.insert('w', Button::FORWARD);
    buttons.insert('a', Button::LEFT);
    buttons.insert('s', Button::LEFT);
}