# How to prepare your Raspberry 

## Burn the image to the microSD
For this I use https://www.balena.io/etcher/

## Enable a new user and boot it
- Mount the partition called "boot"  
- Add user and pass
  - https://www.raspberrypi.com/documentation/computers/configuration.html#configuring-a-user
```
$ openssl passwd -6 # encode the new password
$ vim /media/$USER/boot/userconf.txt # add user:encoded_pass and save
```
- Unmount the MicroSD card  
- Insert it to the raspberry pi  
- Connect it to power and the network (use a cable for now, mate!)  
- Find the IP  
```
$ nmap -sP 192.168.0.0/24  
```

## SSH into it, give it a name, tweak raspbian
```
$ ssh <USER>@\<IP\> # password is whatever you chose previously
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
- \> Exit > Reboot  

## Update, upgrade, install the basics
```
$ sudo apt-get update  
$ sudo apt-get upgrade  
$ sudo apt-get install -y pigpio vim git
```

## Make pigpio daemon run at boot
```
$ sudo systemctl enable pigpiod
$ sudo systemctl start pigpiod
```

## Configure your own user
- Add your SSH key to log in  
```
$ sudo mkdir -p /home/$NEWUSER/.ssh  
$ sudo vi /home/$NEWUSER/.ssh/authorized_keys # Here you should paste your public SSH key and save
```
```
$ sudo chmod 600 /home/$NEWUSER/.ssh/authorized_keys  
$ sudo chown -R $NEWUSER:$NEWUSER /home/$NEWUSER 
```
- Log out, log in again
```
$ logout  
$ ssh -i <PATH TO YOUR SSH KEY> $NEWUSER@<IP>  
```
  
## Strengthen SSH
```
$ sudo vim /etc/ssh/sshd_config   
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
$ sudo apt-get update && sudo apt-get install -y fail2ban  
$ sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local  
$ sudo systemctl restart fail2ban  
```

## Installing and configuring Firewall
- Install and configure UFW  
```
$ sudo apt-get update && sudo apt-get install -y ufw  
```
- Configure default config  
```
$ sudo ufw default deny incoming  
$ sudo ufw default allow outgoing  
$ sudo ufw allow ${SSH_PORT}  
$ sudo ufw enable  
```

# Install Rust
- curl https://sh.rustup.rs -sSf | sh
-----
# Make Raspberry connect to LAN through Wi-Fi
Connect your Wifi dongle.  
Read https://www.raspberrypi.org/documentation/configuration/wireless/wireless-cli.md  
```
$ sudo raspi-config  
```
\> Localisation Options > Change Wi-fi Country > Choose yours  
\> Network Options > Wi-Fi > add the name of the WiFi network and the pass  

