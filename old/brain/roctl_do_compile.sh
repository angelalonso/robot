#!/usr/bin/env bash


rustup override set nightly
rustup update && cargo update

cross build --target=arm-unknown-linux-gnueabihf
