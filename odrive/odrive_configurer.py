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
    ENCODER_CPR = 8192  # counts per revolution for the AMT - 102 Encoder
    NUM_POLES = 24
    VBUS_VOLTAGE = 24
    CAN_BAUD_RATE = 250000


    def __init__(self, axis_num, sensorless = False, _output = True):
        """
        Initalizes RoverMotorConfig class by finding odrive, erase its 
        configuration, and grabbing specified axis object.

        :param axis_num: Which channel/motor on the odrive your referring to.
        :type axis_num: int (0 or 1)
        """
        self.axis_num = axis_num
        self.axis = None
        self.odrv = self.get_odrive(_output)
        self.sensorless = sensorless

        self.output = _output
        # Connect to Odrive

        self.axis = self.get_axis(self.axis_num, self.odrv)

    @classmethod
    def get_odrive(cls, output):
        # connect to Odrive
        print("Looking for ODrive...") if output else None
        odrv = odrive.find_any()
        print("Found ODrive.") if output else None
        return odrv

    @classmethod
    def get_axis(cls, axis_num, odrv):
        return getattr(odrv, f"axis{axis_num}")

    def configure(self, CAN_id):
        # This is to lower the amount of power going back into the odrive when the motor is braking
        self.odrv.config.enable_brake_resistor = True

        self.config_motors()
        self.config_sensorless() if self.sensorless else self.config_encoder()
        self.config_CAN(CAN_id)

        self.odrv.config.brake_resistance = 1
        self.axis.controller.config.input_mode = INPUT_MODE_VEL_RAMP

        self.save()

    def config_motors(self):
        # The motors have a peak current of 9 amps
        self.axis.motor.config.current_lim = 9
        self.axis.motor.config.requested_current_range = 10

        # The speed of the motor will be limited to this speed in [turns/sec]
        self.axis.controller.config.vel_limit = 50
        self.axis.motor.config.pole_pairs = RoverMotorConfig.NUM_POLES / \
            2  # The MN4004 has 24 magnet poles, so 12 pole pairs
        # The MN4004 has an idle current of 0.2 A but we set it at 2 for it to go faster during calibration
        self.axis.motor.config.calibration_current = 2
        # this is specified in the odrive documentation
        self.axis.motor.config.torque_constant = 8.27 / RoverMotorConfig.MOTOR_KV
        # self.axis.motor.config.resistance_calib_max_voltage = 0.4 * VBUS_VOLTAGE ****do not use

        self.axis.motor.config.motor_type = MOTOR_TYPE_HIGH_CURRENT

    def config_encoder(self):
        if self.axis.config.enable_sensorless_mode: return

        self.axis.encoder.config.cpr = RoverMotorConfig.ENCODER_CPR
        #self.axis.encoder.config.mode = ENCODER_MODE_INCREMENTAL
        self.axis.encoder.config.use_index = True

    def config_sensorless(self):
        self.axis.config.enable_sensorless_mode = True

        self.axis.controller.config.vel_gain = 0.01
        self.axis.controller.config.vel_integrator_gain = 0.05
        self.axis.controller.config.control_mode = CONTROL_MODE_VELOCITY_CONTROL

        self.axis.controller.config.vel_limit = 50
        # 5 turns_per_sec / (2 * 3.14159265 * NUM_POLES / 2)
        self.axis.sensorless_estimator.config.pm_flux_linkage = 5.51328895422 / (2 * RoverMotorConfig.NUM_POLES * RoverMotorConfig.MOTOR_KV)

    def config_CAN(self, can_axis_id):
        self.odrv.can.config.baud_rate = RoverMotorConfig.CAN_BAUD_RATE
        self.axis.config.can.node_id = can_axis_id

    def check_error(self, obj):
        if hasattr(obj, "error"):
            if obj.error != 0:
                print("\nERROR \n"
                      "ODrive reported error {}.\n"
                      "Dumping object data:\n{}\n".format(obj.error, obj))
                return False
        return True

    def motor_calib(self):
        print("\nCalibrating Motor {} (You should hear a beep)...".format(self.axis_num))
        self.axis = self.get_axis(self.axis_num, self.get_odrive())
        self.axis.requested_state = AXIS_STATE_MOTOR_CALIBRATION

        # Wait for calibration to take place
        wait_until(lambda : self.axis.motor.config.phase_inductance != 0)
        print("phase inductance: {}".format(self.axis.motor.config.phase_inductance))

        if self.check_error(self.axis.motor):
            self.axis.motor.config.pre_calibrated = True

    def encoder_calib(self, use_index=True, sensorless_enc_error = True):
        self.axis = self.get_axis(self.axis_num, self.get_odrive())
        if use_index:
            self.axis.encoder.config.use_index = True
            print("Rotating Encoder {} back to index...".format(self.axis_num))
            self.axis.requested_state = AXIS_STATE_ENCODER_INDEX_SEARCH
            time.sleep(3)


        print("Calibrating Encoder {} (The motor should rotate back-and-forth)...".format(self.axis_num))
        self.axis.requested_state = AXIS_STATE_ENCODER_OFFSET_CALIBRATION
        #waits until encoder finishes calibrating or throws an error
        wait_until(
            lambda :
            (abs(self.axis.encoder.config.direction) == 1 and
            self.axis.encoder.config.phase_offset != 0) or
            self.axis.encoder.error != 0
            )

        if self.check_error(self.axis.encoder):
            self.axis.encoder.config.pre_calibrated = True
        elif sensorless_enc_error:
            print("Encoder Error, reverting to sensorless mode...")
            self.sensorless = True
            self.config_sensorless()
            #If there is an encoder error (i.e. encoder isn't detected), the motor becomes sensorless
            

    def run_calib(self):
        self.motor_calib()
        self.encoder_calib() if not(self.sensorless) else None
        self.save()

    def save(self):
        try:
            self.get_odrive().save_configuration()
        except ObjectLostError:
            pass

def wait_until(cond, sec_limit: float = 10.0):
    tick = 0
    while not(cond()) and tick < sec_limit:
        time.sleep(0.1)
        tick += 0.1
    print("wait timed out") if tick >= sec_limit else None

def main():
    odrv = RoverMotorConfig.get_odrive()
    print("Erasing pre-existing configuration...")
    try:
        odrv.erase_configuration()
    except:
        pass

    odrv_can_id = 2 * int(input("ODrive CAN ID > "))

    odrive_config1 = RoverMotorConfig(axis_num=0)
    odrive_config1.configure(CAN_id=odrv_can_id)

    odrive_config2 = RoverMotorConfig(axis_num=1)
    odrive_config2.configure(CAN_id=odrv_can_id + 1)
    odrv = RoverMotorConfig.get_odrive()


    input("Make sure the motor is free to move, then press enter...")

    odrive_config1.run_calib()
    odrive_config2.run_calib()

    print("Calibration Complete!")


if __name__ == "__main__":
    main()