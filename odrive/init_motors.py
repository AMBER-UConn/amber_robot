import odrive
from odrive.enums import *

def get_odrive():
    return odrive.find_any()

def get_axis(axis_num, odrv):
    return getattr(odrv, f"axis{axis_num}")

def init(odrv = get_odrive()):
    for i in range(2):
        ax = get_axis(i, odrv)
        ax.requested_state = AXIS_STATE_ENCODER_INDEX_SEARCH


if __name__ == "__main__":
    init()
