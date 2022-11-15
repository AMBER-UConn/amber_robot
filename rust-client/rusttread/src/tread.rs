use crate::{
    utils::ResponseManip as RData
};


enum Direction {
    FORWARD,
    BACKWARD,
}

struct TreadControl {
    can: CANSocket
}


impl TreadControl {
    fn set_velocity(&self, vel: i32) {
        let data = RData::combine_32(vel.)
    }

    fn set_direction(&self, direction: Direction) {
        self.can
    }
}