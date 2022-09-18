
Follow the instructions of the following links to configure the imu:
Fixed Drivers:
- https://learn.sparkfun.com/tutorials/how-to-install-ch340-drivers
- https://github.com/juliagoda/CH341SER/blob/master/readme.txt
1. Clone the github repository and follow the directions in the readme.txt
2. See Issue #1 and #2 to fix the issues mentioned

Other helpful links:
IMU Repo: https://github.com/WITMOTION/HWT905-TTL/blob/master/HWT905%20TTL%20Manual.pdf
Rust serial communication: https://crates.io/crates/serialport

https://www.baeldung.com/linux/all-serial-devices

Notes:
__ Issue #1 Linux not detecting the usb-serial communicator __
Follow the driver installation instructions

__ Issue #2 usb-serial communicator connects but then disconnects instantly __
Ex:
```
[10506.270594] ch34x 1-5.1:1.0: ch34x converter detected
[10506.271416] usb 1-5.1: ch34x converter now attached to ttyUSB0
[10507.444884] input: BRLTTY 6.4 Linux Screen Driver Keyboard as /devices/virtual/input/input42
[10507.566362] usb 1-5.1: usbfs: interface 0 claimed by ch34x while 'brltty' sets config #1
[10507.566900] ch34x ttyUSB0: ch34x converter now disconnected from ttyUSB0
[10507.566916] ch34x 1-5.1:1.0: device disconnected
```

1. Locate your `brltty` configuration location with `ls /usr/lib/udev/rules.d/ | grep "brltty"`
2. Open the file in a text editor and edit the configuration according to this post:
https://unix.stackexchange.com/a/680547
