#!/usr/bin/env bash

source /opt/ros/rolling/setup.bash

export ROS_DOMAIN_ID=4
export ROS_LOCALHOST_ONLY=1

colcon build

. ./install/local_setup.bash

ros2 run circuit_nodes led_action_server &
ros2 run circuit_nodes node_master &
