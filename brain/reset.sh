#!/bin/bash

source .env
# Stop any previous runs
online=false
while [ $online = false ]
do
  echo "trying to reach the robot..."
  ping ${HOST} -c 1
  if [[ $? -eq 0 ]]
    then
      online=true
  fi
done

ssh=false
while [ $online = false ]
do
  echo "trying to reach the robot's SSH..."
  ${SSH_COMM} "echo $PWD"
  if [[ $? -eq 0 ]]
    then
      online=true
  fi
done
${SSH_COMM} "kill \$(ps aux | grep brain | grep setup | awk '{print \$2}')"
# set motors to 0
${SSH_COMM} "cd robot/brain; \
  RUST_LOG=info target/arm-unknown-linux-gnueabihf/debug/brain reset setup_reset.yaml
  "
# # Stop that last one we just triggered
# ${SSH_COMM} "kill \$(ps aux | grep brain | grep cfg | awk '{print \$2}')"

