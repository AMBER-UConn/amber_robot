
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
        // println!("ACCELERATION:");
        get_acc(&mut port);
        // println!("ANGULAR VELOCITY:");
        // get_ang_vel(&mut port);
        // println!("ANGLE (ROLL PITCH YAW):");
        // get_ang(&mut port);
    }
}

fn get_acc(port: &mut Box<dyn SerialPort>) {
    // Requests acceleration info from imu and writes it to output variable
    let mut first_byte = [0u8];
    while first_byte[0] != 0x55 
    {
        port.read_exact(&mut first_byte).expect("failed to read values");
    }

    // keep reading until you find the start bit of 85
        let mut remaining = [0u8; 10];
        port.read_exact(&mut remaining);
        
        if remaining[0] == 0x51 {
            println!("{:?}", remaining);
            let mut calc: u32 = 0x55;
            for val in remaining.iter() {
                calc += *val as u32;
            }
            calc -= remaining[9] as u32; 
            println!("calculated: {} ---- actual: {}", calc as u8, remaining[9]);
            // Prints out accX accY accZ for our viewing pleasure
            for i in (  1..remaining.len() - 2 - 1).step_by(2) {
                let (accL, accH) = (remaining[i] as i32, remaining[i + 1] as i32);
                print!("{:?}, ", (((accH << 8) | accL) as f32 /32768.0 * 16.0 * 9.81));
            }
            // for (i, acc) in &output[2..-2].chunks(2).enumerate() {
            //     let (accL, accH) = (acc[0] as u16, acc[1] as u16);
            //     print!("{:?}, ", (((accH << 8) | accL) as f32 /32768.0 * 16.0));
            // }
            println!("\n");
        }
        println!("{:?}", remaining);

    // let mut single_buf = [0u8;100];
    // println!("{}", port.read(&mut single_buf).unwrap());
    // println!("")
    // println!("{}", ((output[4] as u16) << 8) | (output[3] as u16));
    // Compares checksum to make sure no errors occured
}

fn get_ang_vel(port: &mut Box<dyn SerialPort>) {
    // Requests angular velocity info from imu and writes it to output variable
    let mut first_byte = [0u8];
    while first_byte[0] != 0x55 
    {
        port.read_exact(&mut first_byte).expect("failed to read values");
    }
        let mut remaining = [0u8; 10];
        port.read_exact(&mut remaining);

    // Compares checksum to make sure no errors occured
    if remaining[0] == 0x52 {
        println!("{:?}", remaining);
        let mut calc: u32 = 0x55;
        for val in remaining.iter() {
            calc += *val as u32;
        }
        calc -= remaining[9] as u32;
        println!("calculated: {} ---- actual: {}", calc as u8, remaining[9]);
        // Prints out wX wY wX for our viewing pleasure
        for i in (1..remaining.len()-2-1).step_by(2) {
            let (velL, velH) = (remaining[i] as i32, remaining[i+1] as i32);
            print!("{:?} ", (((velH<<8)|velL)/32768*2000))
        }
    println!("\n");
    }
}

fn get_ang(port: &mut Box<dyn SerialPort>) {
    // Requests angle info from imu and writes it to output variable
    let mut first_byte = [0u8];
    while first_byte[0] != 0x55 
    {
        port.read_exact(&mut first_byte).expect("failed to read values");
    }
        let mut remaining = [0u8; 10];
        port.read_exact(&mut remaining);

    // Compares checksum to make sure no errors occured
    if remaining[0] == 0x52 {
        println!("{:?}", remaining);
        let mut calc: u32 = 0x55;
        for val in remaining.iter() {
            calc += *val as u32;
        }
        calc -= remaining[9] as u32;
        println!("calculated: {} ---- actual: {}", calc as u8, remaining[9]);
        // Prints out ROLL PITCH YAW (degrees) for our viewing pleasure
        for i in (1..remaining.len()-2-1).step_by(2) {
            let (angL, angH) = (remaining[i] as i32, remaining[i+1] as i32);
            print!("{:?} ", (((angH<<8)|angL)/32768*180))
        }
    println!("\n");
    }
}