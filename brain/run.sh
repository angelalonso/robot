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

function crossbuild() {
  rm -rf log/* build/* install/brain/*

  trap ctrl_c INT

#  cwd=$(pwd)
#  cd scripts/robotlogic && \
#  cargo build --release &&
#  cd $cwd
#  
  # compile rust lib with maturin
  CWD=$(pwd)
#  rm -rf build/
  cd scripts/robotlogic
  python3 -m venv env
  source env/bin/activate
## TODO: build this or the other one based on a variable (mode crossbuild?)
## also maybe just build a release and install with pip right after?
#  maturin develop
  maturin build --release --target aarch64-unknown-linux-gnu --zig -i python3.10
  rm robotlogic.whl || true
  cp $BUILT robotlogic.whl
#  pip install robotlogic.whl
  deactivate
  cd ${CWD}

  # TODO: cross build as well
  source /opt/ros/rolling/local_setup.sh
#  . ./interfaces/install/setup.bash && \
  colcon build && \
    . ./install/setup.bash
}

function build() {
  echo "################## BUILD ####################"
  rm -rf log/* build/* install/brain/*

  trap ctrl_c INT

  # Compile rust lib with maturin (also cross-compile)
  # - Avoid doing this on the Raspberry itself (takes long and some Raspis collapse at this point)
  ARCH=$(uname -m)
  if [ ${ARCH} != "aarch64" ]; then
    CWD=$(pwd)
    rm -rf build/
    cd scripts/robotlogic
    python3 -m venv env
    source env/bin/activate
    # Compile for the local computer and for the Raspi
    maturin build --release --zig -i python3.8
    maturin build --release --target aarch64-unknown-linux-gnu --zig -i python3.10
    deactivate
    cd ${CWD}
  fi

  # TODO: try to avoid compiling this on the Raspi too
  source /opt/ros/rolling/local_setup.sh
#  . ./interfaces/install/setup.bash && \
  colcon build && \
    . ./install/setup.bash
}

function just_run() {
  echo "##################  RUN  ####################"
  trap ctrl_c INT

  # Load latest released robotlogic library
  CWD=$(pwd)
  cd scripts/robotlogic
  ARCH=$(uname -m)
  if [ ${ARCH} == "x86_64" ]; then
    pip3 install ${BUILT_X8664}
  elif [ ${ARCH} == "aarch64" ]; then
    pip3 install ${BUILT_AARCH64}
  else
    echo "ERROR: No version of robotlogic for $ARCH found!"
    exit 2
  fi
  cd ${CWD}

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

# TODO: add more architectures here
BUILT_AARCH64="./target/wheels/robotlogic-0.1.0-cp310-cp310-manylinux_2_17_aarch64.manylinux2014_aarch64.whl"
BUILT_X8664="./target/wheels/robotlogic-0.1.0-cp38-cp38-manylinux_2_12_x86_64.manylinux2010_x86_64.whl"
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
elif [[ "$1" == "run" ]]; then
  just_run
else
  just_run
fi
