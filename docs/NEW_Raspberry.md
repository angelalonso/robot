# Installation

## Download the latest Raspberry Pi OS Lite 
```wget https://downloads.raspberrypi.org/raspios_lite_armhf/images/raspios_lite_armhf-2022-04-07/2022-04-04-raspios-bullseye-armhf-lite.img.xz
```
, or check the latest at https://www.raspberrypi.com/software/operating-systems/

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
- Run the preloading script
```
./preloading.sh
```
... and follow instructions

## Boot the Raspberry
- Safely remove the MicroSD from your computer.
- Mount the microSD into the Raspberry
- Connect the Raspberry to your Router with an RJ45 cable
- Power up the Raspberry
- You should see the LED blinking in intervals
- When it's finished, it will blick as follows --- 
