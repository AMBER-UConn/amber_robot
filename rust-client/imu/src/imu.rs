
use std::time::Duration;

use serialport::SerialPort;

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
        let mut byte_read = [0u8; 1];
        loop {
            while byte_read[0] != 0x55 {
                // Requests buffer info from imu and writes it to output variable
                self.port.read_exact(&mut byte_read).expect("Failed to read!");
            }
            self.port.read_exact(&mut byte_read);

            if byte_read[0] == *data_type {break;}
        }
        let mut reading = [0u8; 9]; 
        self.port.read_exact(&mut reading);

        if ! IMU::checksum(&data_type, &reading).0 {
            return Err(format!("Checksum failed! Data is {:?}, Checksum is {:?}, calculated is {:?}",
                                reading,    
                                reading[8], 
                                IMU::checksum(&data_type, &reading).1));
        }
        return Ok(reading);
    }

    fn result_parser(output: [u8; 9], constant: Option<f32>) -> [f32; 3] {
        let mut result = [0f32; 3];
        let c = constant.unwrap_or(1f32);

        for i in (0..output.len() - 2 - 1).step_by(2) {
            let (L, H) = (output[i] as i16, output[i+1] as i16);
            
            result[i/2] = ((H << 8)| L) as f32 / 32768.0 * c;
        }

        return result;
    }

    pub fn get_acc(&mut self) -> Result<[f32; 3], String>  {
        // Requests acceleration info from imu and writes it to output variable
        let output = 
        match self.request(&0x51) {
            Ok(T) => {
                T
            },
            Err(E) => {
                println!("{}", E);
                return Err(E);
            }
        };

        return Ok(IMU::result_parser(output, Some(180.0)));
    }

    pub fn get_ang_vel(&mut self) -> Result<[f32; 3], String>  {
        // Requests angular velocity info from imu and writes it to output variable
        let output = 
        match self.request(&0x52) {
            Ok(T) => {
                T
            },
            Err(E) => {
                println!("{}", E);
                return Err(E);
            }
        };
            
        return Ok(IMU::result_parser(output, Some(2000.0)));

    }

    pub fn get_ang(&mut self) ->  Result<[f32; 3], String> {
        // Requests angle info from imu and writes it to output variable
        let output = 
        match self.request(&0x53) {
            Ok(T) => {
                T
            },
            Err(E) => {
                println!("{}", E);
                return Err(E);
            }
        };

        return Ok(IMU::result_parser(output, Some(180.0)));
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
        println!("{:?}", imu.get_acc().unwrap());
        println!("ANGULAR VELOCITY:");
        println!("{:?}", imu.get_ang_vel().unwrap());
        println!("ANGLE (ROLL PITCH YAW):");
        println!("{:?}", imu.get_ang().unwrap());


    }
}