#!/bin/bash

source .env

git checkout ${DEV_BRANCH}
git add ${ARDUINO_FILES}
git add cfg.yaml
git add move_cfg.yaml
git add src/
git add Cargo.toml
git commit -m "changing and installing"
git push origin ${DEV_BRANCH}


${SSH_COMM} "cd robot/brain; git pull; git checkout ${DEV_BRANCH} && git pull && \
  RUST_LOG=debug ${CARGO} run classic cfg.yaml move_cfg.yaml
  "
