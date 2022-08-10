'''

Script to help make tuning the motor controllers easier.
Some steps are automated, while others 

Reference: https://docs.odriverobotics.com/v/0.5.4/control.html

To run the script, do:
sudo -E python tuner.py

'''

import odrive_configurer as configurer
from odrive_configurer import RoverMotorConfig as RMC
import odrive
from odrive.enums import *
import keyboard as kb
from time import sleep

#from odrive_sim import *

class FORM:
    DEF = "\033[0m"
    BOLD = "\033[1m"
    UNDERL = "\033[4m"
    RED = "\033[91m"
    GREEN = "\033[92m"

def format(text: str, form: FORM = FORM.DEF):
    return form + text + FORM.DEF

def printl(x):
    print("\x1b[2K{}".format(x), end = "\r")


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

    def set_vel_pos(con_id, controller):
        x = int(input(  
                "Set", 
                "input vel" if con_id == 0 else "input pos",
                "> ",
                " " * 10,
                end = "/r"
            ))
        match x:
            case 0: # velocity
                controller.inp_vel = x
            case 1: # position
                controller.inp_pos = x
        
        

    def tuner():
        axis_id = UI.axis_select()
        #axis = 
        axis = RMC(axis_id, _output = False).axis
        con_config = axis.controller.config
        
        control_types = ["vel_gain", "pos_gain"]

        #DEFAULT VALUES
        con_config.pos_gain = 20.0
        con_config.vel_gain = 0.01
        con_config.vel_integrator_gain = 0.05

        con_id = 0 # 0 - Velocity Control, 1 - Position Control
        shift_was_pressed = False
        cl_was_pressed = False
        closed_loop = False
        print("\nCTRL - Increase Value\t"
              "Alt - Decrease Value\t"
              "Shift - Change Control Type + Value to Tune\t"
              "Z - Toggle Control Loop")
        while True:
            #con_id = con_id % 2
            printl("{}\tvel_gain: {:.5f}\tpos_gain: {:.5f}\tclosed loop: {}".format(control_types[con_id],
                                                                                  con_config.vel_gain,
                                                                                  con_config.pos_gain,
                                                                                  closed_loop))

            if kb.is_pressed("ctrl"): #INCREASE
                match con_id:
                    case 0: #vel_gain
                        con_config.vel_gain *= 1.3
                    case 1: #pos_gain
                        con_config.pos_gain *= 1.3
            
            if kb.is_pressed("alt"): #DECREASE
                match con_id:
                    case 0: #vel_gain
                        con_config.vel_gain /= 1.3
                    case 1: #pos_gain
                        con_config.pos_gain /= 1.3
            
            if kb.is_pressed("z"): #CLOSED LOOP
                if not cl_was_pressed:
                    if closed_loop:
                        axis.requested_state = AXIS_STATE_IDLE
                        closed_loop = False
                    else:
                        axis.requested_state = AXIS_STATE_CLOSED_LOOP_CONTROL
                        closed_loop = True
                    cl_was_pressed = True
                    
            else:
                cl_was_pressed = False

            if kb.is_pressed("shift"):
                if not shift_was_pressed: #Increments 1 each type shift is pressed
                    con_id = (con_id + 1) % len(control_types)
                    shift_was_pressed = True
            else:
                shift_was_pressed = False

            if kb.is_pressed("h"):
                if con_id is 0:
                    con_config.vel_gain /= 2

            if kb.is_pressed("s"):
                UI.set_vel_pos(con_id, axis.controller)

            if kb.is_pressed("q"):
                break


            match con_id:
                case 0: #vel_gain
                    con_config.control_mode = CONTROL_MODE_VELOCITY_CONTROL
                case 1: #pos_gain
                    con_config.control_mode = CONTROL_MODE_POSITION_CONTROL
            
            
            sleep(0.1) #Holds UI frame before refreshing
            

        #axis.requested_state = AXIS_STATE_MOTOR_CALIBRATION

        #Calculates vel_integrator_gain = 0.5 * vel_gain * bandwidth (in Hz)
        con_config.vel_integrator_gain = 0.5 * con_config.vel_gain * 1/(axis.encoder.config.bandwidth/1000)
    
        try:
            RMC.get_odrive().save_configuration()
        except configurer.ObjectLostError:
            pass
        print("SAVED!")
        UI.tuner()


if __name__ == "__main__":

    #print("TEST "+format("RED", FORM.RED)+" TEST", end="\r")
    UI.tuner()

    