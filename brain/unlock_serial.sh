#!/usr/bin/env bash

kill $(lsof /dev/ttyUSB0 | tail -n1 | awk '{print $2}')
