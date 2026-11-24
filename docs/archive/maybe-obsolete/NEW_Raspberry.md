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
- First it will run the "cloud-init" part (Don't worry, it's just a name, no cloud involved) 
  - It will take more than 30 minutes to finish, because this step updates the machine among other stuff.
- Then it will run all steps on files/autosetup.sh (feel free to check the script beforehand)
  - You should see the LED blinking in intervals, the amount of blinks show the number of steps left.
- When it doesn't blink anymore, two things may have happened:
  - It finished alright. Unplug+Plug the Raspberry and the robot program should start automatically
    - Base version currently turns Led ON and both motors for 1 second, then stops.
  - It failed. In this case the autosetup.sh script will continue from the latest successful step on (you should see a number of blinks again)
    - At this point it makes sense to SSH into the Raspberry, using the credentials you set on your .env, to troubleshoot.

# Compilation
If you want to build from your laptop for the Raspberry you will need to install aarch64-linux-gnu and zig:
```
sudo apt install gcc make gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
rustup target install aarch64-unknown-linux-gnu
pip install ziglang
```

See brain/run.sh for an example on how to cross compile
