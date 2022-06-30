use rustodrive::{canproxy::CANProxy, odrivegroup::ODriveGroup, messages::ODriveCANFrame};

fn setup_can() {
    let mut can_proxy = CANProxy::new("can1");

    can_proxy.register_rw("thread1", |can_read_write| {
        let odrive = ODriveGroup::new();
        // can_read_write.request(ODriveCANFrame { axis: todo!(), cmd: todo!(), data: todo!() });
    });
    
    can_proxy.register_ro("thread2", |can_read| {
        let odrive = ODriveGroup::new();
    });
}

fn main() {
    setup_can();
}
