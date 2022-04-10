use crate::gamepad::{Button, Gamepad};


pub struct keyboard{
    button_map: HashMap<char, Button>, 
    state: HashMap<char, bool>,
}

impl keyboard{
    pub fn convert_char(&self, c:char)-> Button{
        match self.button_map.get(c) {
            Some(button) => return button,
            None => return Button::UNDEFINED,
        }
    }
    
    fn new(keymap:HashMap<char, Button>)->keyboard{
        return keyboard{
            button_map:keymap,
            state: HashMap::new(),
        }
    }
}

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
    buttons.insert('s', Button::BACKWARD);
    buttons.insert('d', Button::RIGHT);
    buttons.insert('<', Button::CCW);
    buttons.insert('>', Button::CW);
    buttons.insert('t', Button::MODE_SHIFT);
}

#[cfg(tests)]
mod tests{
    #[test]
    fn test_button_conversion(){
        let kb = keyboard::new(default_buttons());
        assert_eq!(kb.convert_char('w'), Button::FORWARD);
    }
}