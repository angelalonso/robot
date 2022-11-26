#!/usr/bin/env bash
#

set -eo pipefail

function do_build() {
  show_log i "################## BUILD ####################"
  trap ctrl_c INT

  # TODO: build on a specific folder for each architecture, link afterwards
  # TODO: do that for each module inside src
  # TODO: create a list of modules to do that for
  # TODO: maybe create a second one for rust modules in the future
  CWDMAIN=$(pwd)
  cd ${CODEPATH}
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
  cd $CWDMAIN

}
function crossbuild() {
  show_log i "##################  CROSS BUILD  ####################"
  trap ctrl_c INT

  rm -rf ../cross_circuits 
  cp -R ../circuits ../cross_circuits

  docker build \
    --build-arg NEWUSER=$USER \
    --platform linux/arm64/v8 \
    -t aarch64-cross .
  docker run \
    --rm \
    --user $USER \
    --platform linux/arm64/v8 \
    -v $PWD/../cross_circuits:/home/$USER/ros2_ws \
    -v "$PWD:/work" \
    -it aarch64-cross \
    /bin/bash -c cd ros2_ws && ./run.sh build
}

function deploy() {
# TODO: 
#  not found: "/home/aaf/ros2_ws/src/action_interfaces/install/local_setup.bash"
#  not found: "/home/aaf/ros2_ws/src/action_interfaces/install/local_setup.bash"
#  not found: "/home/aaf/ros2_ws/src/action_servers/install/local_setup.bash"
# TODO: use rsync ?
  scp -P ${SSHPORT} -r -C ../cross_circuits ${NEWUSER}@${SSHIP}:/home/${NEWUSER}/robot/cross_circuits
}

function just_run() {
  show_log i "##################  RUN  ####################"
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

function build_n_test() {
  build
  just_test
}

function do_all() {
  # TODO: check previous step worked before moving to next
  build
  just_test
}

# AUX FUNCTIONS

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

function show_log {
  TSTAMP=$(date "+%Y-%m-%d %H:%M:%S")
  case $1 in
    "debug"|"d")
      echo "[$TSTAMP][DEBUG] - $2"
      ;;
    "info"|"i")
      echo "[$TSTAMP][INFO] - $2"
      ;;
    "warn"|"w")
      echo "[$TSTAMP][WARN] - $2"
      ;;
    "error"|"err"|"e")
      echo "[$TSTAMP][ERROR] - $2"
      ;;
    *)
      echo "[$TSTAMP][DEBUG] - Wrong Logging mode"
      exit 2
      ;;
  esac
}

function check_dotenv {
  if [ ! -f ".env" ]; then
    show_log e "ERROR! .env file not found!"
    show_log i "REMEMBER you can copy env.template to .env and adapt it!"
    show_log i "Exiting..."
    exit 2
  fi
}

function show_help() {
  show_log i "SYNTAX:"
  show_log i "  build    - Build related packages locally"
  show_log i "  test     - Run tests locally"
  show_log i "  cross    - Cross-build for the robot's architecture"
  show_log i "  deploy   - Deploy current crossbuild to robot"
  show_log i "  run      - Run current crossbuild on the robot"
  show_log i "  rollback - Deploy previous crossbuild version on robot and make it current"
  show_log i "  help     - Show this help"
}

function do_mode() {
  if [[ "$1" == "help" ]]; then
    show_help
  elif [[ "$1" == "build" ]]; then
    do_build
  elif [[ "$1" == "test" ]]; then
    do_test
  elif [[ "$1" == "buildtest" ]] || [[ "$1" == "buildntest" ]] || [[ "$1" == "build_n_test" ]]; then
    do_build_n_test
  elif [[ "$1" == "cross" ]] || [[ "$1" == "crossbuild" ]]; then
    do_crossbuild
  elif [[ "$1" == "deploy" ]]; then
    do_deploy
  elif [[ "$1" == "run" ]]; then
    do_run
  else
    do_all
  fi
}

# Modes
# Default
#   - Build -> Test -> Crossbuild -> Deploy -> Run

ARCH=$(uname -m)
CODEPATH="./circuits"

check_dotenv
do_mode $1
