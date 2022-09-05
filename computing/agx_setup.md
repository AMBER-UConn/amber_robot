# Configuring CAN on the AGX
See [this article](https://medium.com/@ramin.nabati/enabling-can-on-nvidia-jetson-xavier-developer-kit-aaaa3c4d99c9) for a more detailed overview
1. Install `busybox` to change register values
```sudo apt install busybox```

2. The can_setup.sh script should run at the start but if not, you can run it manually

*Note* the pinout of the AGX fallows the Raspi 40 pin standard normally but the following pin configurations are changed
_Pin | Type_
37/29 | CAN RX
33/31 | CAN TX
34/30 | Ground
17/1  | 3.3V

## How to start `can_setup.sh` on startup
1. Create a symlink for the can_setup.sh script onto the root directory
`ln -s ~/amber_robot/computing/can_setup.sh /`
2. Run the enable_can_setup.sh script which should automatically configure the script to run on startup
