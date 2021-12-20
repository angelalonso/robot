#!/usr/bin/env zsh

function ctrl_c() {
  # once the launch run is stopped with CTRL-C we want to clean up
  echo "** Trapped CTRL-C"
}

function build_n_run() {
  trap ctrl_c INT

  source /home/angel.alonso/Downloads/ros2_foxy/ros2-linux/local_setup.zsh
  colcon build && \
    . ./install/setup.zsh && \
    ros2 launch brain brain.launch.py
}

function just_run() {
  trap ctrl_c INT

  source /home/angel.alonso/Downloads/ros2_foxy/ros2-linux/local_setup.zsh
  . ./install/setup.zsh && \
    ros2 launch brain brain.launch.py
}


if [[ "$1" == "build" ]]; then
  build_n_run
else
  just_run
fi
