#!/usr/bin/env bash
if [[ $1 == "" ]]; then
  sudo ip link set lo multicast on
  source /opt/ros/rolling/setup.bash

  export ROS_DOMAIN_ID=4
  export ROS_LOCALHOST_ONLY=1

  colcon build

  . ./install/local_setup.bash

  ros2 run circuit_nodes led_action_server &
  ros2 run circuit_nodes motor_l_action_server &
  ros2 run circuit_nodes motor_r_action_server &
  ros2 run circuit_nodes node_master &
fi
if [[ $1 == "off" ]]; then
  kill $(ps aux | grep robot | grep circuits | awk '{print $2}')
  sudo ip link set lo multicast off
fi
