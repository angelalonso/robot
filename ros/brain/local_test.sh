#!/usr/bin/env zsh

colcon build && \
  . ./install/setup.zsh && \
  ros2 launch brain brain.launch.py
