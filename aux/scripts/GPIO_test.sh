#!/bin/bash

# Set the pin for input
PIN="16"
echo "$PIN"> /sys/class/gpio/export
echo in > /sys/class/gpio/gpio$PIN/direction

# Set the variable through command substitution
b=$(cat /sys/class/gpio/gpio$PIN/value)

# Echo the value to the console
echo "result is $b"

