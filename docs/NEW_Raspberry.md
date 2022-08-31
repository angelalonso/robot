# Installation

## Download the latest Raspberry Pi OS Lite 
```wget https://downloads.raspberrypi.org/raspios_lite_armhf/images/raspios_lite_armhf-2022-04-07/2022-04-04-raspios-bullseye-armhf-lite.img.xz
```
, or check the latest at https://www.raspberrypi.com/software/operating-systems/

## Burn the image to the microSD
- Connect the MicroSD card to your Computer, and burn the OS image you just downloaded
For this I use https://www.balena.io/etcher/

## Enable SSH to the bootloader and boot it - TODO: automate this too
- Mount the MicroSD partition called "boot" (using nautilus, maybe?)
- In ubuntu:  
```
touch /media/$USER/boot/ssh  
```

## Modify and copy over autoconfig file, user and pass... make it work - TODO: automate this too
```
cp ../files/rc.local /media/$USER/rootfs/etc/rc.local 
chmod +x /media/$USER/rootfs/etc/rc.local 
cp ../files/autosetup.sh /media/$USER/rootfs/home/pi/
chmod +x /media/$USER/rootfs/home/pi/autosetup.sh
cp ../files/userconf /media/$USER/boot/
```
- user is tempuser and pass is This_Is_The_First_Password


- ...and safely remove the MicroSD from your computer.

## Boot the Raspberry
- Mount the microSD into the Raspberry
- Connect the Raspberry to your Router with an RJ45 cable
- Power up the Raspberry
- Find your Raspberry (is 192.168.1.0/24 also your network's IP Range?)
```
nmap -sP 192.168.1.0/24
```
- SSH into it
```
ssh tempuser@182.168.1.42 # pass is This_Is_The_First_Password
```
# TODO: Modify this default user and pass from a script
