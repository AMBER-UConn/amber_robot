#!/bin/bash

# Configure the registers to change the pinot of the AGX
sudo busybox devmem 0x0c303000 32 0x0000C400
sudo busybox devmem 0x0c303008 32 0x0000C458
sudo busybox devmem 0x0c303010 32 0x0000C400
sudo busybox devmem 0x0c303018 32 0x0000C458

# Mount the CAN controllers and drivers
sudo modprobe can
sudo modprobe can_raw
sudo modprobe mttcan

# Specify the bitrate
# sudo ip link set can0 type can bitrate <bitrate> dbitrate <payload bitrate> berr-reporting <--- [bus error reporting enabled] on fd <-- [stands for Flexible Data Rate] on

sudo ip link set can0 type can bitrate 500000 dbitrate 2000000 berr-reporting on fd on
sudo ip link set can1 type can bitrate 500000 dbitrate 2000000 berr-reporting on fd on

# Start the can interface
sudo ip link set up can0
sudo ip link set up can1

exit 0