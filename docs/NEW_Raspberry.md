# Installation

## Download the latest Ubuntu for Raspi
Get it from https://ubuntu.com/download/raspberry-pi (64 bits)

## Burn the image to the microSD
- Connect the MicroSD card to your Computer, and burn the OS image you just downloaded
For this I use https://www.balena.io/etcher/

## Run the preloading Script
From the /files folder on this repo:
- Copy env.template to .env
```
cp env.template .env
```
- Modify it to your liking
- IMPORTANT: You MUST change at least USER and PASS
- Mount the two /system-boot and /writable partitions from the MicroSD (e.g.: click on them on nautilus)
- Install whois to make use of mkpasswd
```
sudo apt-get install whois
```
- Run the preloading script
```
./preloading.sh
```
... and follow instructions

## Boot the Raspberry
- Safely remove the MicroSD from your computer.
- Mount the microSD into the Raspberry
- Connect the Raspberry to your Router with an RJ45 cable - IMPORTANT! If you don't, you'll have to restart the process from the "Burn the image" step!
- Power up the Raspberry
- You should see the LED blinking in intervals
- When it's finished, it will blink as follows --- 

# Compilation
If you want to build at your laptop for the Raspberry you will need to install gcc-arm-linux-gnueabihf
```
sudo apt install gcc-arm-linux-gnueabihf # NEEDED?
sudo apt install gcc make gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
rustup target install aarch64-unknown-linux-gnu
pip install ziglang
```
