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

######## 
MOTOR_KV = 300
ENCODER_PPR = 5000 # pulses per revolution
NUM_POLES = 24
VBUS_VOLTAGE = 24

################ Odrive variables. A better system for referencing odrive variables should be put into place
MOTOR_TYPE_HIGH_CURRENT = 0 # this is according to the odrive docs
MOTOR_TYPE_GIMBAL = 2 # this is according to the odrive docs
CONTROL_MODE_VELOCITY_CONTROL = 2 
INPUT_MODE_VEL_RAMP = 2
AXIS_STATE_CLOSED_LOOP_CONTROL = 8

def setup_axes(odrive, axes_list: List):
    odrive.config.enable_brake_resistor = True # This is to lower the amount of power going back into the odrive when the motor is braking
    for ax in axes_list:
        ax.motor.config.current_lim = 9 # The motors have a peak current of 9 amps
        ax.motor.config.requested_current_range = 10

        ax.controller.config.vel_limit = 50 # The speed of the motor will be limited to this speed in [turns/sec]
        ax.motor.config.pole_pairs = NUM_POLES / 2 # The MN4004 has 24 magnet poles, so 12 pole pairs
        ax.motor.config.calibration_current = 2 # The MN4004 has an idle current of 0.2 A but we set it at 2 for it to go faster during calibration
        ax.motor.config.torque_constant = 8.27 / MOTOR_KV # this is specified in the odrive documentation
        # ax.motor.config.resistance_calib_max_voltage = 0.4 * VBUS_VOLTAGE

        odrive.config.brake_resistance = 1

        ax.motor.config.motor_type = MOTOR_TYPE_HIGH_CURRENT
        ### encoder setup
        ax.controller.config.input_mode = INPUT_MODE_VEL_RAMP
        # ax.encoder.config.cpr = 4 * ENCODER_PPR # the count per revolution is 4 * the ppr of the encoder

    odrive.save_configuration()

def setup_sensorless(axis):
    print("this is the latest change 2")
    axis.controller.config.vel_gain = 0.01
    axis.controller.config.vel_integrator_gain = 0.05
    axis.controller.config.control_mode = CONTROL_MODE_VELOCITY_CONTROL

    axis.controller.config.vel_limit = 50 # 5 turns_per_sec / (2 * 3.14159265 * NUM_POLES / 2)
    axis.sensorless_estimator.config.pm_flux_linkage = 5.51328895422 / (2 * NUM_POLES * MOTOR_KV)
    axis.config.enable_sensorless_mode = True



# def setup_sensorless(odrive, axes_list):
#     for ax in axes_list:
#         odrv0.axis0.controller.config.vel_gain = 0.01
#         odrv0.axis0.controller.config.vel_integrator_gain = 0.05
#         odrv0.axis0.controller.config.control_mode = CONTROL_MODE_VELOCITY_CONTROL
#         odrv0.axis0.controller.config.vel_limit = <a value greater than axis.config.sensorless_ramp.vel / (2 * 3.14159265 * POLE_PAIRS)>
#         odrv0.axis0.motor.config.current_lim = 2 * odrv0.axis0.config.sensorless_ramp.current
#         odrv0.axis0.sensorless_estimator.config.pm_flux_linkage = 5.51328895422 / (NUM_POLES * MOTOR_KV)
#         odrv0.axis0.config.enable_sensorless_mode = True

CAN_BAUD_RATE = 500000
def setup_can(odrive, axes):
    odrive.can.config.baud_rate = CAN_BAUD_RATE
    for ax_id, ax in enumerate(axes):
        ax.config.can.node_id = ax_id
    odrive.save_configuration()
