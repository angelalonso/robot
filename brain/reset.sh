#!/bin/bash

source .env
# Stop any previous runs
${SSH_COMM} "kill \$(ps aux | grep brain | grep cfg | awk '{print \$2}')"
# set motors to 0
${SSH_COMM} "cd robot/brain; \
  RUST_LOG=debug ${CARGO} run test move_cfg_stop.yaml
  "
# Stop that last one we just triggered
${SSH_COMM} "kill \$(ps aux | grep brain | grep cfg | awk '{print \$2}')"

