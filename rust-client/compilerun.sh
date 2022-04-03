#!/bin/bash
source /opt/ros/foxy/setup.sh

roscore
cargo build --release
./target/release/amber_robot