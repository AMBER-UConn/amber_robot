pub trait Gamepad{
    fn send(&self);
    fn recieve(&self);

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
