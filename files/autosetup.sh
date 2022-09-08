#!/usr/bin/env bash
## TODO:
# Check before running each step (is not installed? install!)
# error with pip

## -------------- Vars

CONFIGFILE=".env"

## -------------- Step Functions

function change_hostname {
  ./blink.sh 9 &
  PID="$!"

  sudo hostname $HOSTNAME
  if [ $? -ne 0 ]; then
    show_log err "There was an error setting up hostname"
  fi
  sudo sed -i -e "s/raspberrypi/${HOSTNAME}/g" /etc/hosts
  if [ $? -ne 0 ]; then
    show_log err "There was an error writing to /etc/hosts"
  fi
  sudo sed -i -e "s/raspberrypi/${HOSTNAME}/g" /etc/hostname
  if [ $? -ne 0 ]; then
    show_log err "There was an error writing to /etc/hostname"
  fi
  show_log info "hostname change done"

  kill $PID
  ./blink.sh 0
}

function change_locale {
  show_log info "locales TBD"
  # locale -a
  # locale-gen en_US.utf8
  # locale-gen de_DE.utf8
  # /etc/default/locale
  # update-locale LANG=en_US.utf8
}
## - tweak raspbian  
## ```
## $ sudo raspi-config  
## ```
## - \> Localisation Options > Change Locale > choose the Locales you need, hit OK  
## - \> Exit > Reboot  
## 
function config_user {
  ./blink.sh 8 &
  PID="$!"

  mkdir -p /home/$USER/.ssh  
  if [ $? -ne 0 ]; then
    show_log err "There was an error creating /home/$USER/.ssh"
  fi
  cat /autosetup/sshpubkey | tee /home/$USER/.ssh/authorized_keys
  if [ $? -ne 0 ]; then
    show_log err "There was an error adding pubkey"
  fi
  sudo chmod 600 /home/$USER/.ssh/authorized_keys  
  if [ $? -ne 0 ]; then
    show_log err "There was an error changing access to authorized_keys"
  fi
  sudo chown -R $USER:$USER /home/$USER 
  if [ $? -ne 0 ]; then
    show_log err "There was an error changing owner to /home/$USER"
  fi
  show_log info "adding SSH key done"
  # sudo adduser $USER dialout gpio
  #echo "adding USER to dialout and gpio key done"

  kill $PID
  ./blink.sh 0
}
## ## Add your own admin user, remove user pi
## ```
## $ sudo useradd -s /bin/bash -m -d /home/$NEWUSER $NEWUSER
## ```
## - Give that user a strong password  
## ```
## $ sudo passwd $NEWUSER
## ```
## - Add your SSH key to log in  
## ```
## $ sudo mkdir -p /home/$NEWUSER/.ssh  
## $ sudo vi /home/$NEWUSER/.ssh/authorized_keys # Here you should paste your public SSH key and save
## ```
## - Add your user to the same groups as pi is in  
## ```
## $ vigr
## ```
## - (This can probably be achieved in some other way) Let you user manage GPIO
## ```
## $ sudo adduser $NEWUSER dialout gpio
## ```
## ```
## :%s/:ubuntu/:ubuntu,$NEWUSER/g
## ```
## - Make your NEW USER owner of its own environment  
## ```
## $ sudo chmod 600 /home/$NEWUSER/.ssh/authorized_keys  
## $ sudo chown -R $NEWUSER:$NEWUSER /home/$NEWUSER 
## ```
## - Log out, log in again
## ```
## $ logout  
## $ ssh -i <PATH TO YOUR SSH KEY> $NEWUSER@<IP>  
## ```
## - Get rid of pi  
## ```
## $ sudo deluser -remove-home ubuntu
## ```
##   
function secure_ssh {
  ./blink.sh 7 &
  PID="$!"

  sudo sed -i -e "s/#Port 22/Port ${SSHPORT}/g" /autosetup/sshd_config
  if [ $? -ne 0 ]; then
    show_log err "There was an error setting the new SSH Port"
  fi
  sudo cp /autosetup/sshd_config /etc/ssh/sshd_config
  if [ $? -ne 0 ]; then
    show_log err "There was an error copying ssh_config"
  fi
  sudo systemctl reload ssh  
  if [ $? -ne 0 ]; then
    show_log err "There was an error reloading ssh"
  fi
  show_log info "SSH strengthened"

  kill $PID
  ./blink.sh 0
}
## ## Strengthen SSH
## ```
## $ sudo vim /etc/ssh/sshd_config   
## ```
## ```ChallengeResponseAuthentication no
## PasswordAuthentication no  
## UsePAM no  
## PermitRootLogin no  
## Port $NEWPORT  
## ```
## ```
## $ sudo systemctl reload ssh  
## ```

function update_upgrade {
  ./blink.sh 6 &
  PID="$!"

  sudo apt-get update  
  if [ $? -ne 0 ]; then
    show_log err "There was an error updating packages"
  fi
  sudo apt-get -y upgrade  
  if [ $? -ne 0 ]; then
    show_log err "There was an error upgrading packages"
  fi
  show_log info "updated and upgraded packages"

  kill $PID
  ./blink.sh 0
}
## ## Update, upgrade, install python 3.8 and the basics
## ```
## $ sudo apt-get update  
## $ sudo apt-get upgrade  

function install_packages {
  ./blink.sh 5 &
  PID="$!"

  sudo apt-get install -y vim git rpi.gpio-common python3-pigpio pigpio-tools
  if [ $? -ne 0 ]; then
    show_log err "There was an error installing the base packages"
  fi
  show_log info "installed base packages"

  kill $PID
  ./blink.sh 0
}
## # Install some further packages
## 
## ```
## $ sudo apt-get update  
## $ sudo apt-get install vim git rpi.gpio-common python3-pigpio pigpio-tools # we'll need them later
## $ sudo reboot
## ```
## 

function pigpio_at_boot {
  ./blink.sh 4 &
  PID="$!"

  sudo cp /autosetup/pigpiod.service /etc/systemd/system/
  if [ $? -ne 0 ]; then
    show_log err "There was an error copying pigpiod service file to systemd"
  fi
  sudo systemctl enable pigpiod
  if [ $? -ne 0 ]; then
    show_log err "There was an error enabling pigpiod"
  fi
  sudo systemctl start pigpiod
  if [ $? -ne 0 ]; then
    show_log err "There was an error starting pigpiod"
  fi
  show_log info "enabled pigpiod at boot"

  kill $PID
  ./blink.sh 0
}
## # Make pigpio daemon run at boot
## - Copy files/pigpiod.service to /etc/systemd/system/
## ```
## $ sudo systemctl enable pigpiod
## $ sudo systemctl start pigpiod
## ```
## - 
## 
function install_python {
  ./blink.sh 3 &
  PID="$!"

  wget -O - https://raw.githubusercontent.com/tvdsluijs/raspberry-pi-python-sh-installer/main//python3.9.7.sh | sudo bash
  if [ $? -ne 0 ]; then
    show_log err "There was an error compiling python"
  fi
  curl -O https://bootstrap.pypa.io/get-pip.py
  if [ $? -ne 0 ]; then
    show_log err "There was an error downloading get-pip"
  fi
  python get-pip.py pip==20.3.4
  if [ $? -ne 0 ]; then
    show_log err "There was an error installing pip"
  fi
  rm -rf /usr/bin/lsb_release

  python -m pip install --upgrade pip
  if [ $? -ne 0 ]; then
    show_log err "There was an error upgrading pip"
  fi
  pip install flatdict maturin 
  if [ $? -ne 0 ]; then
    show_log err "There was an error installing stuff with pip"
  fi
  show_log info "installed python and some packages"

  kill $PID
  ./blink.sh 0
}
## $ wget -O - https://raw.githubusercontent.com/tvdsluijs/raspberry-pi-python-sh-installer/main//python3.9.7.sh | sudo bash
## $ pip3 --trusted-host pypi.org --trusted-host files.pythonhosted.org  install flatdict flask maturin 
## ```
##   
function install_fail2ban {
  ./blink.sh 2 &
  PID="$!"

  apt-get install -y fail2ban  
  if [ $? -ne 0 ]; then
    show_log err "There was an error installing fail2ban"
  fi
  cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local  
  if [ $? -ne 0 ]; then
    show_log err "There was an error copying config for fail2ban"
  fi
  systemctl restart fail2ban  
  if [ $? -ne 0 ]; then
    show_log err "There was an error restarting fail2ban"
  fi
  show_log info "installed and configured fail2ban"

  kill $PID
  ./blink.sh 0
}
## - Install fail2ban  
## ```
## $ sudo apt-get install fail2ban  
## $ sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local  
## $ sudo systemctl restart fail2ban  
## ```
## 
function install_firewall {
  ./blink.sh 1 &
  PID="$!"

  echo
  sudo apt-get update && sudo apt-get install -y ufw  
  if [ $? -ne 0 ]; then
    show_log err "There was an error installing ufw"
  fi
  ufw default deny incoming  
  if [ $? -ne 0 ]; then
    show_log err "There was an error denying incoming traffic"
  fi
  ufw default allow outgoing  
  if [ $? -ne 0 ]; then
    show_log err "There was an error allowing outgoing traffic"
  fi
  ufw allow ${SSH_PORT}  
  if [ $? -ne 0 ]; then
    show_log err "There was an error allowing SSH Port"
  fi
  ufw enable  
  if [ $? -ne 0 ]; then
    show_log err "There was an error enabling ufw"
  fi
  show_log info "installed and configured ufw"

  kill $PID
  ./blink.sh 0
}
## ## Installing and configuring Firewall
## - Install and configure UFW  
## ```
## $ sudo apt-get update && sudo apt-get install ufw  
## ```
## - Add support for IPv6  
## ```
## $ sudo vim /etc/default/ufw    
## ```
## ```
## IPV6=yes  
## ```  
## - Configure default config  
## ```
## $ sudo ufw default deny incoming  
## $ sudo ufw default allow outgoing  
## $ sudo ufw allow ${SSH_PORT}  
## $ sudo ufw enable  
## ```
## 
## 
function install_ros2 {
  echo
}
## # Install ROS2
## 
## Follow the Official Guide at https://docs.ros.org/en/galactic/Installation.html
## 
function install_rust {
  echo
}
## # Install Rust
## Follow the Official Guide at https://www.rust-lang.org/tools/install
function connect_wifilan {
  echo
}
## # Make Raspberry connect to LAN through Wi-Fi
## Connect your Wifi dongle.  
## Read https://www.raspberrypi.org/documentation/configuration/wireless/wireless-cli.md  
## ```
## $ sudo raspi-config  
## ```
## \> Localisation Options > Change Wi-fi Country > Choose yours  
## \> Network Options > Wi-Fi > add the name of the WiFi network and the pass  
## 

## -------------- Aux Functions

function show_log {
  TSTAMP=$(date "+%Y-%m-%d %H:%M:%S")
  case $1 in
    "debug"|"d")
      echo "[$TSTAMP][DEBUG] - $2"
      ;;
    "info"|"i")
      echo "[$TSTAMP][INFO] - $2"
      ;;
    "warn"|"w")
      echo "[$TSTAMP][WARN] - $2"
      ;;
    "error"|"err"|"e")
      echo "[$TSTAMP][ERROR] - $2"
      exit 2
      ;;
    *)
      echo "[$TSTAMP][DEBUG] - Wrong Logging mode"
      exit 2
      ;;
  esac
}

function load_dotenv {
  if [ -f "$CONFIGFILE" ]; then
    export $(cat ${CONFIGFILE} | grep -v '^#' | xargs) >/dev/null
  else 
    show_log err ".env file does not exist. Have you created it from env.template?"
  fi
}

function run {
  load_dotenv
  change_hostname
  #change_locale
  config_user
  secure_ssh
  update_upgrade
  install_packages
  pigpio_at_boot
  install_python
  install_fail2ban
  install_firewall
  #install_ros2
  #install_rust 
  #connect_wifilan

## -------------- Main

run

