use std::collections::HashMap;
use crate::gamepad::{Button, Gamepad};


pub struct Keyboard{
    button_map: HashMap<char, Button>, 
    state: HashMap<char, bool>,
}

impl Keyboard{
    pub fn convert_char(&self, c: &char)-> Button{
        match self.button_map.get(c) {
            Some(button) => return button.clone(),
            None => return Button::UNDEFINED,
        }
    }
    
    fn new(keymap:HashMap<char, Button>)->Keyboard{
        return Keyboard{
            button_map:keymap,
            state: HashMap::new(),
        }
    }
}

impl Gamepad for Keyboard{
    
    fn send(&self){
        //todo
    }

    fn recieve(&self){
        //todo
    }

    
}

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
    use crate::keyboard::Keyboard;
    use crate::keyboard::default_buttons;
    use crate::gamepad::Button;

    #[test]
    fn test_button_conversion(){
        let kb = Keyboard::new(default_buttons());
        assert_eq!(kb.convert_char(&'w'), Button::FORWARD);
    }
}