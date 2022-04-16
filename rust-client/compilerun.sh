#!/bin/bash

. /opt/ros/$ROS_DISTRO/setup.sh
cargo test

tail -f /dev/null # this keeps the rust container running indefinitely
# if you want to run the code do
# cargo run

# To run the release version
# cargo build --release
# ./target/release/amber_robot