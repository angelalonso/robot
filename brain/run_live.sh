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

function read_dotenv() {
  while read line
  do
    TYPE=${line:0:3}
    if [[ "${TYPE}" == "LED" ]]
    then
      LEDNR=$(echo $line | awk -F '=' '{print $2}')
      LEDPINS+=("${LEDNR}")
    fi
  done < .env
}

function turn_leds_off() {
  for pin in "${LEDPINS[@]}"
  do
    echo " - Making sure PIN Nr $pin is OFF..."
    # export
    if [ ! -e $BASE_GPIO_PATH/gpio$pin ]; then
      echo "$pin" > $BASE_GPIO_PATH/export
    fi    
    # set pin to output
    echo "out" > $BASE_GPIO_PATH/gpio$pin/direction
    # switch off
    echo 0 > $BASE_GPIO_PATH/gpio$pin/value
  done
}

# Use non-mocked python nodes
cp scripts/node_arduino.py.prod scripts/node_arduino.py
if [[ "$1" == "build" ]]; then
  build_n_run
else
  just_run
fi

LEDPINS=()
BASE_GPIO_PATH=/sys/class/gpio
read_dotenv
turn_leds_off
