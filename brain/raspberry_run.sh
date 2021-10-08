#!/usr/bin/env bash

function ctrl_c() {
  # once the launch run is stopped with CTRL-C we want to clean up
  echo "** Trapped CTRL-C"

}

trap ctrl_c INT

colcon build && \
  . ./install/setup.bash && \
  ros2 launch brain brain.launch.py

