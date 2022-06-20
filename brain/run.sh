#!/usr/bin/env bash

function ctrl_c() {
  # once the launch run is stopped with CTRL-C we want to clean up
  echo "** Trapped CTRL-C"

#  # Set back to real libraries to have them ready to be pushed to the repo
#  sed -i 's/^#import RPi.GPIO as GPIO/import RPi.GPIO as GPIO/g' scripts/*.py
#  sed -i 's/^from fake_rpi import fake_rpi as GPIO/#from fake_rpi import fake_rpi as GPIO/g' scripts/*.py
#
#  # Use production python nodes
#  cp scripts/node_arduino.py.prod scripts/node_arduino.py
}

function adapt() {
  # Set fake libraries for laptop development
  sed -i 's/^import RPi.GPIO as GPIO/#import RPi.GPIO as GPIO/g' scripts/*.py
  sed -i 's/^#from fake_rpi import fake_rpi as GPIO/from fake_rpi import fake_rpi as GPIO/g' scripts/*.py

  # Use mocked python nodes
  #    Comment out this line if you test your arduino connected to your laptop
  cp scripts/node_arduino.py.test scripts/node_arduino.py
}

function build_n_run() {
  trap ctrl_c INT

  # TODO: use maturin maybe
  #cwd=$(pwd)
  #cd scripts/rustbrain && \
  #cargo build --release &&
  #cd $cwd
  
  # compile rust lib with maturin
  CWD=$(pwd)
  cd scripts/robotlogic
  source env/bin/activate
  maturin develop
  deactivate
  cd ${CWD}

  source /opt/ros/rolling/local_setup.sh
  . ./interfaces/install/setup.bash && \
  colcon build && \
    . ./install/setup.bash && \
    ros2 launch brain brain.launch.py
}

function just_run() {
  trap ctrl_c INT

  source /opt/ros/rolling/local_setup.sh
  . ./interfaces/install/setup.bash && \
  . ./install/setup.bash && \
    ros2 launch brain brain.launch.py
}


#adapt
if [[ "$1" == "build" ]]; then
  build_n_run
else
  just_run
fi
