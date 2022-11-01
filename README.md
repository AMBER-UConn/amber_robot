# The UConn AMBER (Adaptive Morphing and Balanced Exploratory Rover) Project

A group of students at the University of Connecticut were selected as finalists in the annual NASA Big Idea Challenge ([see our submission](https://bigidea.nianet.org/wp-content/uploads/2022-BIG-Idea-Challenge-Finalist-Team-Synopses.pdf) or the [video](https://youtu.be/4zF1PQumCn8))

### Modules and project structure

`kinematics`:

`odrive`: contains scripts to configure ODrive motor controllers for use with project hardware alongside tuning. 

`computing`: defines shell scripts to configure computing devices used in the project (Raspbery Pi 4, Isolated CAN Hat, NVIDIA AGX)

`docker`: contains dockerfile for project virtualization in a container for development purposes. Additionally used to standardize the ROS environment.

`rust-client`: serves as the main entry point for the project and application.\
  |-- `rustodrive`\
  |--|-- `gui`: implements a user interface for visualizing the state of odrives and executing simple commands for debugging\
  |--|-- `rustodrive`: implements communication between the Raspberry Pi 4 and the ODrives via CAN\
  |--|-- `imu`: implemented serial communication with [HWT905-TTL](https://github.com/WITMOTION/HWT905-TTL)

`tread`: communication between the arduino and the main computing setup. Eventually will be integrated into `rust-client`.