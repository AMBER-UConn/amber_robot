
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

    fn checksum(data_type: &u8, data: &[u8; 9]) -> (bool, u8) {
        let checksum = data[8];
        let mut rcvd_total: u32 = 0x55 + *data_type as u32;

        for item in data {
            rcvd_total += *item as u32;
        }
        rcvd_total -= data[8] as u32;

        (rcvd_total as u8 == checksum, rcvd_total as u8)
    }

    fn request(&mut self, data_type: &u8) -> Result<[u8; 9], String> {
        
        //let _ = twos_complement();
        let mut byte_read = [0u8; 1];
        loop {
            while byte_read[0] != 0x55 {
                // Requests buffer info from imu and writes it to output variable
                //self.port.write(buffer).expect("Failed to write!");
                self.port.read_exact(&mut byte_read).expect("Failed to read!");
                //print!("{:?}", byte_read);
            }
            self.port.read_exact(&mut byte_read);

            if byte_read[0] == *data_type {break;}
        }
        let mut reading = [0u8; 9]; 
        self.port.read_exact(&mut reading);


        //println!("{:?}", output);
        //println!("{:?}", output.split_last().unwrap().1.iter().map(|x| *x as u32).sum::<u32>().to_be_bytes());

        // 85 dec = 0x55 hex
        if ! IMU::checksum(&data_type, &reading).0 {
            return Err(format!("Checksum failed! Data is {:?}, Checksum is {:?}, calculated is {:?}",
                                reading,    
                                reading[8], 
                                IMU::checksum(&data_type, &reading).1));
        }
        //assert!(output[0] == 0x55 && 
        //        IMU::check_checksum(output), "Checksum failed! Data is {:?}, Checksum is {:?}, calculated is {:?}",
        //                                      output,    
        //                                      output[10], 
        //                                      IMU::checksum(output));
        return Ok(reading);
    }

    pub fn get_acc(&mut self) {
        // Requests acceleration info from imu and writes it to output variable
        let output = 
        match self.request(&0x51) {
            Ok(T) => {
                T
            },
            Err(E) => {
                println!("{}", E);
                return;
            }
        };

        // println!("{}", ((output[4] as u16) << 8) | (output[3] as u16));

            // Prints out accX accY accZ for our viewing pleasure
            for i in (0..output.len() - 2 - 1).step_by(2) {
                let (accL, accH) = (output[i] as i16, output[i + 1] as i16);
                print!("{:?}, ", (((accH << 8) | accL) as f32 / 32768.0 * 16.0 * 9.81));
            }
            // for (i, acc) in &output[2..-2].chunks(2).enumerate() {
            //     let (accL, accH) = (acc[0] as u16, acc[1] as u16);
            //     print!("{:?}, ", (((accH << 8) | accL) as f32 /32768.0 * 16.0));
            // }
        println!("\n");
    }

    pub fn get_ang_vel(&mut self) {
        // Requests angular velocity info from imu and writes it to output variable
        let output = 
        match self.request(&0x52) {
            Ok(T) => {
                T
            },
            Err(E) => {
                println!("{}", E);
                return;
            }
        };

            // Prints out wX wY wX for our viewing pleasure
            for i in (0..output.len() - 2 - 1).step_by(2) {
                let (velL, velH) = (output[i] as i16, output[i+1] as i16);
                print!("{:?} ", (((velH<<8)|velL) as f32 / 32768.0 * 2000.0))
            }
        println!("\n");
    }

    pub fn get_ang(&mut self) {
        // Requests angle info from imu and writes it to output variable
        let output = 
        match self.request(&0x53) {
            Ok(T) => {
                T
            },
            Err(E) => {
                println!("{}", E);
                return;
            }
        };

        // Compares checksum to make sure no errors occured

            // Prints out ROLL PITCH YAW (degrees) for our viewing pleasure
            for i in (0..output.len() - 2 - 1).step_by(2) {
                let (angL, angH) = (output[i] as i16, output[i+1] as i16);
                print!("{:?} ", (((angH<<8)|angL) as f32 / 32768.0 * 180.0))
            }
        println!("\n");
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