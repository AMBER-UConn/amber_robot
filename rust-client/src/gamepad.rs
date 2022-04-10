pub trait Gamepad{
    fn send(&self);
    fn recieve(&self);
}

pub enum Button{
    FORWARD,
    BACKAWARD,
    LEFT,
    RIGHT,
    CCW,
    CW,
    MODE_SHIFT,
}