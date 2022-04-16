pub trait GamepadSender{
    fn send_message(&self);
}
pub trait GamepadReciever {
    fn get_message(&self);
    fn translate(&self, keypress: &char) -> Button;
    fn has_message(&self);
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Button{
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
    CCW,
    CW,
    MODE_SHIFT,
    UNDEFINED
}
