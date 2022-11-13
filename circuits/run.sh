#!/usr/bin/env bash
#


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

function crossbuild() {
  echo
}

function build() {
  echo "################## BUILD ####################"
  trap ctrl_c INT

  source /opt/ros/rolling/local_setup.sh
  rm -rf log/* build/* install/*

  CWD=$(pwd)
  cd src/action_interfaces
  rm -rf log/* build/* install/*
#  colcon build
  colcon build && \
  . ./install/setup.bash
  cd $CWD
  cd src/action_servers
  rm -rf log/* build/* install/*
#  colcon build
  colcon build && \
  . ./install/setup.bash
  cd $CWD
  cd src/circuit_nodes
  rm -rf log/* build/* install/*
  colcon build
#  colcon build && \
#  . ./install/setup.bash
  cd $CWD

}

function just_run() {
  echo "##################  RUN  ####################"
  trap ctrl_c INT

  source /opt/ros/rolling/local_setup.sh

  CWD=$(pwd)
  cd src/action_interfaces
  . ./install/setup.bash && \
  cd $CWD
  cd src/action_servers
  . ./install/setup.bash && \
  cd $CWD
  cd src/circuit_nodes
  . ./install/setup.bash && \
    ros2 launch circuit_nodes circuits.launch.py
  cd $CWD
}

function build_n_run() {
  build
  just_run
}

function check_dotenv {
  if [ ! -f ".env" ]; then
    echo "ERROR! .env file not found!"
    echo "  REMEMBER you can copy env.template to .env and adapt it!"
    echo "Exiting..."
    exit 2
  fi
}

check_dotenv

if [[ "$1" == "help" ]]; then
  echo "SYNTAX:"
  echo "  build    - Build related packages"
  echo "  buildrun - Build related packages, and then run"
  echo "  run      - Run whatever was built before"
elif [[ "$1" == "crossbuild" ]]; then
  crossbuild
elif [[ "$1" == "build" ]]; then
  build
elif [[ "$1" == "buildrun" ]]; then
  build_n_run
elif [[ "$1" == "buildnrun" ]]; then
  build_n_run
elif [[ "$1" == "build_n_run" ]]; then
  build_n_run
elif [[ "$1" == "run" ]]; then
  just_run
else
  just_run
fi
