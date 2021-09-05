#!/usr/bin/env bash

#cross build --target=arm-unknown-linux-gnueabihf
cargo build --release

perf record --call-graph dwarf -- cargo run --release test setup.yaml

perf report
