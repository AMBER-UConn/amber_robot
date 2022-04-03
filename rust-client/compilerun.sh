#!/bin/bash
. /opt/ros/$ROS_DISTRO/setup.sh && roscore & 
cargo build --release
./target/release/amber_robot

tail -f /dev/null