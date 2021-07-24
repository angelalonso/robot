#!/usr/bin/env bash

cargo profiler cachegrind --release -n 6 --sort dr
