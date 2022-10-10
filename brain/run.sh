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

function build() {
  trap ctrl_c INT

  cwd=$(pwd)
  cd scripts/robotlogic && \
  cargo build --release &&
  cd $cwd
  
  # compile rust lib with maturin
  CWD=$(pwd)
  rm -rf build/
  cd scripts/robotlogic
  python3 -m venv env
  source env/bin/activate
  maturin develop
  deactivate
  rm robotlogic.so || true
  BUILT=$(ls ./env/lib/python*/site-packages/robotlogic/*.so)
  cp $BUILT robotlogic.so
  cd ${CWD}
}

function just_run() {
  trap ctrl_c INT

  source /opt/ros/rolling/local_setup.sh
  . ./install/setup.bash && \
    ros2 launch brain brain.launch.py
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

#adapt
if [[ "$1" == "build" ]]; then
  build
elif [[ "$1" == "help" ]]; then
  echo "SYNTAX:"
  echo "  build    - Build related packages"
  echo "  buildrun - Build related packages, and then run"
  echo "  run      - Run whatever was built before"
elif [[ "$1" == "buildrun" ]]; then
  build_n_run
elif [[ "$1" == "run" ]]; then
  just_run
else
  just_run
fi
