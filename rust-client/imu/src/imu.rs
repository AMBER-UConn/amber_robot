
use std::time::Duration;

use serialport::SerialPort;

//fn twos_complement() -> i32 {
//    1
//}
pub struct IMU {
    port: Box<dyn SerialPort>
}

impl IMU {

    pub fn new(port_path: Option<&str>, baud_rate: Option<u32>) -> IMU {
        let p = port_path.unwrap_or("/dev/ttyUSB0");
        let b_r = baud_rate.unwrap_or(9600);

        IMU {
            port: serialport::new(p, b_r)
                  .timeout(Duration::from_millis(1000))
                  .open()
                  .expect("Failed to open port")
        }
    }

    fn request(&mut self, buffer: &[u8]) -> [u8; 11] {
        // Requests buffer info from imu and writes it to output variable
        self.port.write(buffer).expect("Failed to write");
        //let _ = twos_complement();
        let mut output = [0u8;11];
        self.port.read_exact(&mut output);
        return output;
    }

    pub fn get_acc(&mut self) {
        // Requests acceleration info from imu and writes it to output variable
        let output = self.request(&[0x51]);

        // println!("{}", ((output[4] as u16) << 8) | (output[3] as u16));

        // Compares checksum to make sure no errors occured
        if output[1] == 0x51 {
            println!("{:?}", output);
            let mut calc: i32 = -1 * output[10] as i32;
            for val in output.iter() {
                calc += *val as i32;
            }
                
            println!("calculated: {} ---- actual: {}", calc >> 2, output[10]);

            // Prints out accX accY accZ for our viewing pleasure
            for i in (  2..output.len() - 2 - 1).step_by(2) {
                let (accL, accH) = (output[i] as u16, output[i + 1] as u16);
                println!("{:?}, ", (((accH << 8) | accL) as f32 /32768.0 * 16.0 * 9.81));
            }
            // for (i, acc) in &output[2..-2].chunks(2).enumerate() {
            //     let (accL, accH) = (acc[0] as u16, acc[1] as u16);
            //     print!("{:?}, ", (((accH << 8) | accL) as f32 /32768.0 * 16.0));
            // }
        println!("\n");
        }
        
    }

    pub fn get_ang_vel(&mut self) {
        // Requests angular velocity info from imu and writes it to output variable
        let output = self.request(&[0x52]);

        // Compares checksum to make sure no errors occured
        if output[1] == 0x52 {
            println!("{:?}", output);
            let mut calc: i32 = -1 * output[10] as i32;
            for val in output.iter() {
                calc += *val as i32;
            }

            println!("calculated: {} ---- actual: {}", calc >> 2, output[10]);

            // Prints out wX wY wX for our viewing pleasure
            for i in (2..output.len()-2-1).step_by(2) {
                let (velL, velH) = (output[i] as u16, output[i+1] as u16);
                println!("{:?} ", (((velH<<8)|velL)/32768*2000))
            }
        println!("\n");
        }
    }

    pub fn get_ang(&mut self) {
        // Requests angle info from imu and writes it to output variable
        let output = self.request(&[0x53]);

        // Compares checksum to make sure no errors occured
        if output[1] == 0x53 {
            let mut calc: i32 = -1 * output[10] as i32;
            for val in output.iter() {
                calc += *val as i32;
            }

            println!("calculated: {} ---- actual: {}", calc >> 2, output[10]);

            // Prints out ROLL PITCH YAW (degrees) for our viewing pleasure
            for i in (2..output.len()-2-1).step_by(2) {
                let (angL, angH) = (output[i] as u16, output[i+1] as u16);
                println!("{:?} ", (((angH<<8)|angL)/32768*180))
            }
        println!("\n");
        }
    }
}


pub fn test() {
    //let mut port = serialport::new("/dev/ttyUSB0", 9600)
    //    .timeout(Duration::from_millis(1000))
    //    .open()
    //    .expect("Failed to open port");

    let mut imu = IMU::new(None, None);

     loop {
        // let mut serial_buf: Vec<u8> = vec![0; 32];
        // port.read(serial_buf.as_mut_slice())
        //     .expect("Found no data!");
        // println!("{:?}", serial_buf);
        println!("ACCELERATION:");
        imu.get_acc();
        println!("ANGULAR VELOCITY:");
        imu.get_ang_vel();
        println!("ANGLE (ROLL PITCH YAW):");
        imu.get_ang();
    }
}