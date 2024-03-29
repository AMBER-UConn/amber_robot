
use std::time::Duration;

use serialport::SerialPort;

pub struct IMU {
    port: Box<dyn SerialPort>,
    gravity: f32
}

impl IMU {


    pub fn new(port_path: &str, baud_rate: &u32) -> IMU {
        IMU {
            port: serialport::new(port_path, *baud_rate)
                  .timeout(Duration::from_millis(1000))
                  .open()
                  .expect("Failed to open port"),
            gravity: 9.81
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
                // Keeps reading IMU data until it hits 0x55, the start of the data
                self.port.read_exact(&mut byte_read).expect("Failed to read!");
            }
            // Checks the next byte after 0x55; the data_type of the data
            self.port.read_exact(&mut byte_read);

            // Checks if the data_type is what we want, and breaks from loop if it is
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

    pub fn acceleration(&mut self) -> Result<[f32; 3], String>  {
        // Requests acceleration info from imu and writes it to output variable
        let output = 
        match self.request(&0x51) {
            Ok(T) => {
                T
            },
            Err(E) => {
                //println!("{}", E);
                return Err(E);
            }
        };

        return Ok(IMU::result_parser(output, Some(16.0 * self.gravity)));
    }

    pub fn angular_velocity(&mut self) -> Result<[f32; 3], String>  {
        // Requests angular velocity info from imu and writes it to output variable
        let output = 
        match self.request(&0x52) {
            Ok(T) => {
                T
            },
            Err(E) => {
                //println!("{}", E);
                return Err(E);
            }
        };
            
        return Ok(IMU::result_parser(output, Some(2000.0)));

    }

    pub fn angle(&mut self) ->  Result<[f32; 3], String> {
        // Requests angle info from imu and writes it to output variable
        let output = 
        match self.request(&0x53) {
            Ok(T) => {
                T
            },
            Err(E) => {
                //println!("{}", E);
                return Err(E);
            }
        };

        return Ok(IMU::result_parser(output, Some(180.0)));
    }

    
}


#[cfg(test)]
mod tests {
    use super::IMU;

    #[test]
    fn test_angl_vel(){
        let test_result: [u8; 9] = [10, 0, 247, 255, 0, 0, 247, 9, 167];
        //let test_input: [u8; 11] = [0x55, 0x52, 10, 0, 247, 255, 0, 0, 247, 9, 167];
        
        let actual_output = IMU::result_parser(test_result, Some(2000.0));
        
        let test_output: [f32; 3] = [0.61035156, -0.5493164, 0.0];


        assert!(actual_output == test_output);

    }

    #[test]
    fn test_angle(){
        let test_result: [u8; 9] = [47, 0, 192, 10, 201, 207, 197, 70, 68];
        
        let actual_output = IMU::result_parser(test_result, Some(180.0));

        let test_output: [f32; 3] = [0.2581787, 15.1171875, -67.802124];

        
        assert!(actual_output == test_output);

    }

}