# How to prepare your Raspberry 

## Burn the image to the microSD
For this I use https://www.balena.io/etcher/

## Enable SSH to the bootloader and boot it
- Mount the partition called "boot"  
- In ubuntu:  
```
$ touch /media/$USER/boot/ssh  
```
- Unmount the MicroSD card  
- Insert it to the raspberry pi  
- Connect it to power and the network (use a cable, mate!)  
- Find the IP  
```
$ nmap -sP 192.168.0.0/24  
```

## SSH into it, give it a name, tweak raspbian
```
$ ssh pi@\<IP\> # password is raspberry, accept authenticity
```
- once in, change hostname to your liking  
```
$ sudo hostname \$HOSTNAME  
$ sudo vim /etc/hosts # change raspberrypi to \$HOSTNAME  
$ sudo vim /etc/hostname # change raspberrypi to \$HOSTNAME  
```
- tweak raspbian  
```
$ sudo raspi-config  
```
- \> Localisation Options > Change Locale > choose the Locales you need, hit OK  
- \> Advanced Options > Expand Filesystem  
- \> Exit > Reboot  

## Add your own admin user, remove user pi
```
$ sudo useradd -s /bin/bash -m -d /home/$NEWUSER $NEWUSER
```
- Give that user a strong password  
```
$ sudo passwd $NEWUSER
```
- Add your SSH key to log in  
```
$ sudo mkdir -p /home/$NEWUSER/.ssh  
$ sudo vi /home/$NEWUSER/.ssh/authorized_keys # Here you should paste your public SSH key and save
```
- Add your user to the same groups as pi is in  
```
$ vigr
```
```
:%s/:pi/:pi,$NEWUSER/g
```
- Make your NEW USER owner of its own environment  
```
$ sudo chmod 600 /home/$NEWUSER/.ssh/authorized_keys  
$ sudo chown -R $NEWUSER:$NEWUSER /home/$NEWUSER 
```
- Log out, log in again
```
$ logout  
$ ssh -i <PATH TO YOUR SSH KEY> $NEWUSER@<IP>  
```
- Get rid of pi  
```
$ sudo deluser -remove-home pi  
```
  
## Update, upgrade, install the basics
```
$ sudo apt-get update  
$ sudo apt-get upgrade  
$ sudo apt-get install git vim  
```
  
## Strengthen SSH
```
$ sudo vi /etc/ssh/sshd_config   
```
```ChallengeResponseAuthentication no
PasswordAuthentication no  
UsePAM no  
PermitRootLogin no  
Port $NEWPORT  
```
```
$ sudo systemctl reload ssh  
```
- Install fail2ban  
```
$ sudo apt-get install fail2ban  
$ sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local  
$ sudo systemctl restart fail2ban  
```

## Installing and configuring Firewall
- Install and configure UFW  
```
$ sudo apt-get update && sudo apt-get install ufw  
```
- Add support for IPv6  
```
$ sudo vim /etc/default/ufw    
```
```
IPV6=yes  
```  
- Configure default config  
```
$ sudo ufw default deny incoming  
$ sudo ufw default allow outgoing  
$ sudo ufw allow ${SSH_PORT}  
$ sudo ufw enable  
```

# Clone this repo
```
$ git clone https://github.com/angelalonso/robot
```

# Install and configure avrdude on Raspberry 
See [README.md](README.md) for physical connection scheme

```
$ sudo apt-get update  
$ sudo apt-get install vim git pkg-config libudev1 libudev-dev # we'll need them later
$ sudo apt-get install bison flex -y  
$ wget http://download.savannah.gnu.org/releases/avrdude/avrdude-6.2.tar.gz  
$ tar xfv avrdude-6.2.tar.gz  
$ cd avrdude-6.2/  
$ ./configure --enable-linuxgpio  
$ make  
$ sudo make install  
$ sudo vim /usr/local/etc/avrdude.conf  
```
  
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
  
```
$ sudo avrdude -c linuxgpio -p atmega328p -v  
```

## test an installation of an actual program
```
$ sudo avrdude -c linuxgpio -p atmega328p -v -U flash:w:arduino/001_test_pong/001_test_pong.ino.hex:i
```

# Give your user access to the GPIO pins
, assuming your user is on the gpio Group...   
```
$ sudo chgrp gpio /sys/class/gpio/export  
$ sudo chgrp gpio /sys/class/gpio/unexport  
```

# Prepare Raspberry to run rust
```
$ export RUSTUP_UNPACK_RAM=200000000  # For Raspberry pi 1 REV2 Model B
$ export RUSTUP_UNPACK_RAM=220000000  # For Raspberry pi 1 B+
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
```

# Make Raspberry connect to LAN through Wi-Fi
Connect your Wifi dongle.  
Read https://www.raspberrypi.org/documentation/configuration/wireless/wireless-cli.md  
```
$ sudo raspi-config  
```
\> Localisation Options > Change Wi-fi Country > Choose yours  
\> Network Options > Wi-Fi > add the name of the WiFi network and the pass  

# MORE INFO
## GPIO Map for Raspberry pi 1 B+ 
(thanks to https://github.com/tvierb/raspberry-ascii)
```
                           .___.              
                  +3V3---1-|O O|--2--+5V
          (SDA)  GPIO2---3-|O O|--4--+5V
         (SCL1)  GPIO3---5-|O O|--6--_
    (GPIO_GLCK)  GPIO4---7-|O O|--8-----GPIO14 (TXD0)
                      _--9-|O.O|-10-----GPIO15 (RXD0)
    (GPIO_GEN0) GPIO17--11-|O O|-12-----GPIO18 (GPIO_GEN1)
    (GPIO_GEN2) GPIO27--13-|O O|-14--_
    (GPIO_GEN3) GPIO22--15-|O O|-16-----GPIO23 (GPIO_GEN4)
                  +3V3--17-|O O|-18-----GPIO24 (GPIO_GEN5)
     (SPI_MOSI) GPIO10--19-|O.O|-20--_
     (SPI_MOSO) GPIO9 --21-|O O|-22-----GPIO25 (GPIO_GEN6)
     (SPI_SCLK) GPIO11--23-|O O|-24-----GPIO8  (SPI_C0_N)
                      _-25-|O O|-26-----GPIO7  (SPI_C1_N)
       (EEPROM) ID_SD---27-|O O|-28-----ID_SC Reserved for ID EEPROM
                GPIO5---29-|O.O|-30--_
                GPIO6---31-|O O|-32-----GPIO12
                GPIO13--33-|O O|-34--_
                GPIO19--35-|O O|-36-----GPIO16
                GPIO26--37-|O O|-38-----GPIO20
                      _-39-|O O|-40-----GPIO21
                           '---'
(_ means Ground)
```

# OLDER INFO
## GPIO Map for Raspberry pi 1 REV2 Model B 

```
           .___.              
    5v---1-|O O|--2--3.3v  
    5v---3-|O O|--4--2 SDA  
   GND---5-|O O|--6--3 SCL  
14 TXD---7-|O O|--8--4  
15 RXD---9-|O O|-10--GND  
    18--11-|O O|-12--17  
   GND--13-|O O|-14--27  
    23--15-|O O|-16--22  
    24--17-|O O|-18--3.3v  
   GND--19-|O O|-20--10 MOSI  
    25--21-|O O|-22--9 MISO  
     8--23-|O O|-24--11 SCKL  
     7--25-|O O|-26--GND  
           '---'

```

# Challenges
- The documentation has not been fully tested.
- The process could use automation.
