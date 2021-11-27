#!/usr/bin/env bash

function ctrl_c() {
  # once the launch run is stopped with CTRL-C we want to clean up
  echo "** Trapped CTRL-C"

}

function build_n_run() {
  trap ctrl_c INT

  colcon build && \
    . ./install/setup.bash && \
    ros2 launch brain brain.launch.py
}

function just_run() {
  trap ctrl_c INT

  . ./install/setup.bash && \
    ros2 launch brain brain.launch.py
}


# Use non-mocked python nodes
cp scripts/node_arduino.py.prod scripts/node_arduino.py
if [[ "$1" == "build" ]]; then
  build_n_run
else
  just_run
fi

