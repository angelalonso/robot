#!/bin/bash

source .env
./reset.sh

git checkout ${DEV_BRANCH}
git add ${ARDUINO_FILES}
git add setup.yaml
git add move_cfg.yaml
git add src/
git add actions/
git add Cargo.toml
git commit -m "changing and installing"
git push origin ${DEV_BRANCH}

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
${SSH_COMM} "cd robot/brain; git pull; git checkout ${DEV_BRANCH} && git pull && \
  RUST_LOG=info ${CARGO} run live setup.yaml
  "
