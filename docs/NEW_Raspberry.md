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
# TODO: modify this to use user-data file (among others, check ubuntu docs)
#  First identify steps that can go there (user, pass, ssh key...)
#  Then put the rest on a different side 
#  TODO: how exactly can I run a script on first boot only?
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
- When it's finished, it will blink as follows --- 
