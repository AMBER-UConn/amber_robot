'''

Script to help make tuning the motor controllers easier.
Some steps are automated, while others 

Reference: https://docs.odriverobotics.com/v/0.5.4/control.html

'''

import odrive_configurer as configurer
from odrive_configurer import RoverMotorConfig as RMC
import odrive
from odrive.enums import *

class FORM:
    DEF = "\033[0m"
    BOLD = "\033[1m"
    UNDERL = "\033[4m"
    RED = "\033[91m"
    GREEN = "\033[92m"

def format(text: str, form: FORM = FORM.DEF):
    return form + text + FORM.DEF

def printl(x):
    print(x, end = "\r")


class UI:

    # TODO: menu w/ option to configure
    #def main():
    #    print("")

    def _true_false_str(cond: bool, trustr: str = "", falstr: str = ""):
        return format(trustr, FORM.GREEN) if cond else format(falstr, FORM.RED)

    def axis_select():
        def axis_desc(ax, id = 0):
            has_enc = (ax.encoder.config.phase_offset != 0) and (ax.encoder.error == 0)
            enc_cal = (ax.encoder.config.pre_calibrated)
            return "Axis {}\n".format(id) + \
                   UI._true_false_str(has_enc, "ENCODER FOUND\n", "ENCODER NOT FOUND\n") + \
                   UI._true_false_str(enc_cal, "ENCODER CALIBRATED\n", "ENCODER NOT CALIBRATED\n")
        
        
        #ax = (
        #    RMC(0).axis, RMC(1).axis
        #)

        print("Select Axis to tune:\n")

        for x in range(2):
            print("{})\n".format(x) + axis_desc(RMC(x, _output = False).axis, x))

        sel = -1
        while sel not in range(2):
            sel = int(input("> "))

        return sel



    def tuner():
        axis_id = UI.axis_select()
        axis = RMC(axis_id, _output = False).axis
        
        #axis.requested_state = AXIS_STATE_MOTOR_CALIBRATION



if __name__ == "__main__":

    #print("TEST "+format("RED", FORM.RED)+" TEST", end="\r")
    UI.tuner()

    