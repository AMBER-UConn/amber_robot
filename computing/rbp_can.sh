BITRATE=250000

sudo ip link set can0 up type can bitrate $BITRATE
sudo ip link set can1 up type can bitrate $BITRATE
sudo ifconfig can0 txqueuelen 65536
sudo ifconfig can1 txqueuelen 65536