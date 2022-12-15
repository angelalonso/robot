#!/usr/bin/env bash

function ctrl_c() {
  # once the launch run is stopped with CTRL-C we want to clean up
  echo "** Trapped CTRL-C"

  # Cleanup of PIDs
  for i in $(ps aux | grep target | grep led_action_server | awk '{print $2}'); do kill $i;done
  for i in $(ps aux | grep target | grep action_clients | awk '{print $2}'); do kill $i;done

  exit 2

}

function cargo_build() {
  trap ctrl_c INT

  CWD=$(pwd)

  cd led_action_server/ && cargo build
  cd $CWD
  cd action_clients/ && cargo build
  cd $CWD
}

function cargo_run() {
  trap ctrl_c INT

  CWD=$(pwd)

  cd led_action_server/ && cargo run &
  echo PID: $!
  cd $CWD

  cd action_clients/ && cargo run  # last one must not go to background
  echo PID: $!
  cd $CWD
}

cargo_build
cargo_run
