#!/bin/bash

. /opt/ros/$ROS_DISTRO/setup.sh
cargo test
# if you want to run the code do
# cargo run

# To run the release version
# cargo build --release
# ./target/release/amber_robot