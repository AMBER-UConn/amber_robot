"""This files should be run to either reset or setup an odrive that uses 2 Antigravity motors

https://store.tmotor.com/goods.php?id=438
https://docs.odriverobotics.com/v/latest/getting-started.html

Before running the script, make sure you activate `odrivetool` and then
add this python package to the list of modules at runtime 

```
import sys
sys.path.insert(1, "/home/nvidia/amber_robot/odrive") # 0 is the odrivetool script path, 1 is the place we want to put our package

import setup
setup.setup_axes(odrv0, [odrv0.axis0, odrv0.axis1])
```
"""

from typing import List
from odrive.enums import *
import odrive
from fibre.libfibre import ObjectLostError
import time
import sys

class RoverMotorConfig:
    """
    Class for configuring an Odrive axis for a Amber motor. 
    Only works with one Odrive at a time.
    """
    ######## 
    MOTOR_KV = 300
    ENCODER_PPR = 5000 # pulses per revolution
    NUM_POLES = 24
    VBUS_VOLTAGE = 24
    CAN_BAUD_RATE = 250000

    def __init__(self, axis_num):
        """
        Initalizes RoverMotorConfig class by finding odrive, erase its 
        configuration, and grabbing specified axis object.
        
        :param axis_num: Which channel/motor on the odrive your referring to.
        :type axis_num: int (0 or 1)
        """
        self.axis_num = axis_num
        self.axis = None
    
        # Connect to Odrive
        print("Looking for ODrive...")
        self._find_odrive()
        print("Found ODrive.")

    def _find_odrive(self):
        # connect to Odrive
        self.odrv = odrive.find_any()
        self.axis = getattr(self.odrv, f"axis{self.axis_num}")

    def configure(self, CAN_id):
        # Erase pre-exsisting configuration
        print("Erasing pre-existing configuration...")
        try:
            self.odrv.erase_configuration()
        except:
            pass

        self._find_odrive()

        self.odrv.config.enable_brake_resistor = True # This is to lower the amount of power going back into the odrive when the motor is braking
        
        self.config_motors()
        self.config_sensorless()
        self.config_CAN(CAN_id)

        self.odrv.config.brake_resistance = 1
        self.axis.controller.config.input_mode = INPUT_MODE_VEL_RAMP

        try:
            self.odrv.save_configuration()
        except ObjectLostError:
            pass


    def config_motors(self):
        self.axis.motor.config.current_lim = 9 # The motors have a peak current of 9 amps
        self.axis.motor.config.requested_current_range = 10

        self.axis.controller.config.vel_limit = 50 # The speed of the motor will be limited to this speed in [turns/sec]
        self.axis.motor.config.pole_pairs = RoverMotorConfig.NUM_POLES / 2 # The MN4004 has 24 magnet poles, so 12 pole pairs
        self.axis.motor.config.calibration_current = 2 # The MN4004 has an idle current of 0.2 A but we set it at 2 for it to go faster during calibration
        self.axis.motor.config.torque_constant = 8.27 / RoverMotorConfig.MOTOR_KV # this is specified in the odrive documentation
        # self.axis.motor.config.resistance_calib_max_voltage = 0.4 * VBUS_VOLTAGE ****do not use

        self.axis.motor.config.motor_type = MOTOR_TYPE_HIGH_CURRENT

    def config_encoder(self):
        ### encoder setup
        # self.axis.encoder.config.cpr = 4 * ENCODER_PPR # the count per revolution is 4 * the ppr of the encoder
        pass

    def config_sensorless(self):
        self.axis.controller.config.vel_gain = 0.01
        self.axis.controller.config.vel_integrator_gain = 0.05
        self.axis.controller.config.control_mode = CONTROL_MODE_VELOCITY_CONTROL

        self.axis.controller.config.vel_limit = 50 # 5 turns_per_sec / (2 * 3.14159265 * NUM_POLES / 2)
        self.axis.sensorless_estimator.config.pm_flux_linkage = 5.51328895422 / (2 * RoverMotorConfig.NUM_POLES * RoverMotorConfig.MOTOR_KV)
        self.axis.config.enable_sensorless_mode = True

    def config_CAN(self, can_axes_id):
        self.odrv.can.config.baud_rate = RoverMotorConfig.CAN_BAUD_RATE
        self.axis.config.can.node_id = can_axes_id

    def run_motor_calib(self):
        input("Make sure the motor is free to move, then press enter...")
        
        print("Calibrating Odrive for motor (you should hear a "
        "beep)...")
        self._find_odrive()
        
        self.axis.requested_state = AXIS_STATE_MOTOR_CALIBRATION
        
        # Wait for calibration to take place
        time.sleep(10)
        # TODO the calibration sequence does not appear to work properly

        if self.axis.motor.error != 0:
            print("Error: Odrive reported an error of {} while in the state " 
            "AXIS_STATE_MOTOR_CALIBRATION. Printing out Odrive motor data for "
            "debug:\n{}".format(self.axis.motor.error, 
                                self.axis.motor))
            
            sys.exit(1)

        # If all looks good, then lets tell ODrive that saving this calibration 
        # to persistent memory is OK
        self.axis.motor.config.pre_calibrated = True

if __name__ == "__main__":
    odrive_config1 = RoverMotorConfig(axis_num = 0)
    odrive_config1.configure(CAN_id=0)
    
    odrive_config2 = RoverMotorConfig(axis_num = 1)
    odrive_config2.configure(CAN_id=1)

    odrive_config1.run_motor_calib()

    