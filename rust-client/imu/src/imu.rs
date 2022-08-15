
use std::time::Duration;

use serialport::SerialPort;

pub fn test() {
    let mut port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    loop {
        // let mut serial_buf: Vec<u8> = vec![0; 32];
        // port.read(serial_buf.as_mut_slice())
        //     .expect("Found no data!");
        // println!("{:?}", serial_buf);
        get_acc(&mut port)
    }
}

fn get_acc(port: &mut Box<dyn SerialPort>) {
    port.write(&[0x34]).expect("Failed to write");

    let mut output = [0u8; 32];
    port.read_exact(&mut output);
    println!("{:?}", output);
    // for acc in 0..output.len() - 1 {
    //     let (accL, accH) = (output[acc] as i16, output[acc + 1] as i16);
    //     print!("{:?}, ", (((accH << 8) | accL) as f32 /32768.0 * 16.0));
    // }
    // println!("\n");
}
