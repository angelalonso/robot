#!/usr/bin/env zsh

function ctrl_c() {
  # once the launch run is stopped with CTRL-C we want to clean up
  echo "** Trapped CTRL-C"
  # Set back to real libraries to have them ready to be pushed to the repo
  sed -i 's/^#import RPi.GPIO as GPIO/import RPi.GPIO as GPIO/g' scripts/*.py
  sed -i 's/^from fake_rpi import fake_rpi as GPIO/#from fake_rpi import fake_rpi as GPIO/g' scripts/*.py
}

# Set fake libraries for laptop development
sed -i 's/^import RPi.GPIO as GPIO/#import RPi.GPIO as GPIO/g' scripts/*.py
sed -i 's/^#from fake_rpi import fake_rpi as GPIO/from fake_rpi import fake_rpi as GPIO/g' scripts/*.py

trap ctrl_c INT

colcon build && \
  . ./install/setup.zsh && \
  ros2 launch brain brain.launch.py

