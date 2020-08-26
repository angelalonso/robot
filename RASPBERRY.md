# How to prepare your Raspberry 

## Burn the image to the microSD
For this I use https://www.balena.io/etcher/

## Enable SSH to the bootloader and boot it
- Mount the partition called "boot"  
- In ubuntu:  
$ touch /media/$USER/boot/ssh  
- Unmount the MicroSD card  
- Insert it to the raspberry pi  
- Connect it to power and the network (use a cable, mate!)  
- Find the IP  
$ nmap -sP 192.168.0.0/24  

## SSH into it, give it a name, tweak raspbian
$ ssh pi@<IP> # password is raspberry, accept authenticity
- once in, change hostname to your liking  
$ sudo hostname $HOSTNAME
$ sudo vim /etc/hosts # change raspberrypi to $HOSTNAME
$ sudo vim /etc/hostname # change raspberrypi to $HOSTNAME
- tweak raspbian  
$ sudo raspi-config  
- > Localisation Options > Change Locale > choose the Locales you need, hit OK  
- > Advanced Options > Expand Filesystem  
- > Exit > Reboot  

## Add your own admin user, remove user pi
$ sudo useradd -s /bin/bash -m -d /home/$NEWUSER $NEWUSER
- Give that user a strong password  
$ sudo passwd $NEWUSER
- Add your SSH key to log in  
$ sudo mkdir -p /home/$NEWUSER/.ssh  
$ sudo vi /home/$NEWUSER/.ssh/authorized_keys # Here you should paste your public SSH key and save
- Add your user to the same groups as pi is in  
$ vigr
```
:%s/:pi/:pi,$NEWUSER/g
```
- Make your NEW USER owner of its own environment  
$ sudo chmod 600 /home/$NEWUSER/.ssh/authorized_keys  
$ sudo chown -R $NEWUSER:$NEWUSER /home/$NEWUSER 
- Log out, log in again
$ logout  
$ ssh -i <PATH TO YOUR SSH KEY> $NEWUSER@<IP>  
- Get rid of pi  
$ sudo deluser -remove-home pi  
  
## Update, upgrade, install the basics
$ sudo apt-get update  
$ sudo apt-get upgrade  
$ sudo apt-get install git vim  
  
## Strengthen SSH
$ sudo vi /etc/ssh/sshd_config   
```ChallengeResponseAuthentication no
PasswordAuthentication no  
UsePAM no  
PermitRootLogin no  
Port $NEWPORT  
```
$ sudo systemctl reload ssh  
- Install fail2ban  
$ sudo apt-get install fail2ban  
$ sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local  
$ sudo systemctl restart fail2ban  

## Installing and configuring Firewall
- Install and configure UFW  
$ sudo apt-get update && sudo apt-get install ufw  
- Add support for IPv6  
$ sudo vim /etc/default/ufw    
```
IPV6=yes  
```  
- Configure default config  
$ sudo ufw default deny incoming  
$ sudo ufw default allow outgoing  
$ sudo ufw allow ${SSH_PORT}  
$ sudo ufw enable  

# Clone this repo
git clone https://github.com/angelalonso/robot

# Install and configure avrdude on Raspberry 
See [README.md](README.md) for physical connection scheme

sudo apt-get update  
sudo apt-get install vim git pkg-config libudev1 libudev-dev # these are not needed for avrdude but we'll need them later on
sudo apt-get install bison flex -y  
wget http://download.savannah.gnu.org/releases/avrdude/avrdude-6.2.tar.gz  
tar xfv avrdude-6.2.tar.gz  
cd avrdude-6.2/  
./configure --enable-linuxgpio  
make  
sudo make install  
sudo vim /usr/local/etc/avrdude.conf  
  
```programmer  
  id    = "linuxgpio";  
  desc  = "Use the Linux sysfs interface to bitbang GPIO lines";  
  type  = "linuxgpio";  
  reset = 4;  
  sck   = 11;  
  mosi  = 10;  
  miso  = 9;  
;  
```
  
sudo avrdude -c linuxgpio -p atmega328p -v  

## test an installation of an actual program
sudo avrdude -c linuxgpio -p atmega328p -v -U flash:w:arduino/001_test_pong/001_test_pong.ino.hex:i

# Prepare Raspberry to run rust
export RUSTUP_UNPACK_RAM=200000000  # For Raspberry pi 1 REV2 Model B
export RUSTUP_UNPACK_RAM=220000000  # For Raspberry pi 1 B+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  

