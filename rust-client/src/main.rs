use rustodrive::{
    canproxy::CANProxy,
    casts::Temperature,
    odrivegroup::ODriveGroup,
    response::{ErrorResponse, Success},
    state::{AxisState::*, ControlMode, InputMode},
    threads::ReadWriteCANThread,
    utils::ResultAll,
};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::error::Error;

fn odrive_main(can_read_write: ReadWriteCANThread) {
    // Specify the CAN ids of all odrives connected
    let odrv = ODriveGroup::new(can_read_write, &[0, 1, 2, 3, 4, 5]);

    // Since the return type of axis()/all_axes() is not known at compile time,
    // You must specify the type so the code can cast the type of the response at runtime
    // The short way
    let res0 = odrv.all_axes::<Temperature, _>(|ax| ax.get_temperatures());

    // Or the long way
    let res1: Vec<Result<Success<Temperature>, ErrorResponse>> =
        odrv.all_axes(|ax| ax.get_temperatures());

    // If you don't want to handle the results one by one, you can use the `ResultAll` trait
    // which implements .unwrap_all() or .expect_all() for a Vec<T>
    let res2: Vec<Success<Temperature>> = odrv.all_axes(|ax| ax.get_temperatures()).unwrap_all();

    // If the command does not return anything, such as when setting state, use the () type
    let res3: Vec<Success<()>> = odrv.all_axes(|ax| ax.set_state(ClosedLoop)).unwrap_all();

    // If you want to send a command one at a time, use
    odrv.axis::<(), _>(&0, |ax| {
        ax.motor
            .set_control_mode(ControlMode::PositionControl, InputMode::PosFilter)
    })
    .unwrap();
    odrv.axis::<(), _>(&0, |ax| ax.motor.set_input_pos(180.0 / 360.0))
        .unwrap();
}

// This is useful code to stop threads and exit peacefully
fn main() -> Result<(), Box<dyn Error>> {
    let mut can_proxy = CANProxy::new("can0");

    // We register a thread that is capable of reading state, but also modifying it
    // We can also register a thread that can send "read only" commands.
    can_proxy.register_rw("thread 1", odrive_main);
    can_proxy.register_ro("read only thread", |read_only| {});

    // Turn on the thread to process CAN commands from various threads
    let stop_all = can_proxy.begin();

    // Handle ctrl-c to exit
    let mut signals = Signals::new(&[SIGINT])?;
    for sig in signals.forever() {
        println!("\nQuitting the program {:?}", sig);
        break;
    }

    // Use the hook from `can_proxy.begin()` to clean up the registered threads
    stop_all().unwrap();
    println!("all done!");
    Ok(())
}
