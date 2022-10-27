#!/usr/bin/env bash

./blink.sh 3 &
PID="$!"

sleep 10

kill $PID
./blink.sh 0
