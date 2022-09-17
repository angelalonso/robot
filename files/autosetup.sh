#!/usr/bin/bash

## -------------- Vars

CONFIGFILE="/.env"

## -------------- Step Functions

function config_user {
  /autosetup/blink.sh 8 &
  PID="$!"

  cat /autosetup/sshpubkey | tee /home/$USER/.ssh/authorized_keys
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error adding pubkey"
  fi
  sudo chmod 600 /home/$USER/.ssh/authorized_keys  
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error changing access to authorized_keys"
  fi
  sudo chown -R $USER:$USER /home/$USER 
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error changing owner to /home/$USER"
  fi
  show_log info "adding SSH key done"
  # sudo adduser $USER dialout gpio
  #echo "adding USER to dialout and gpio key done"

  kill $PID
  /autosetup/blink.sh 0
}

function secure_ssh {
  /autosetup/blink.sh 7 &
  PID="$!"

  sudo sed -i -e "s/#Port 22/Port ${SSHPORT}/g" /autosetup/sshd_config
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error setting the new SSH Port"
  fi
  sudo cp /autosetup/sshd_config /etc/ssh/sshd_config
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error copying ssh_config"
  fi
  sudo systemctl reload ssh  
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error reloading ssh"
  fi
  show_log info "SSH strengthened"

  kill $PID
  /autosetup/blink.sh 0
}


function pigpio_at_boot {
  /autosetup/blink.sh 4 &
  PID="$!"

  sudo cp /autosetup/pigpiod.service /etc/systemd/system/
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error copying pigpiod service file to systemd"
  fi
  sudo systemctl enable pigpiod
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error enabling pigpiod"
  fi
  sudo systemctl start pigpiod
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error starting pigpiod"
  fi
  show_log info "enabled pigpiod at boot"

  kill $PID
  /autosetup/blink.sh 0
}

function config_python {
  /autosetup/blink.sh 3 &
  PID="$!"


  python3 -m pip install --upgrade pip
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error upgrading pip"
  fi
  pip install flatdict maturin python-dotenv flask
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error installing stuff with pip"
  fi
  show_log info "installed pip and some packages"

  kill $PID
  /autosetup/blink.sh 0
}

function install_fail2ban {
  /autosetup/blink.sh 2 &
  PID="$!"

  cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local  
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error copying config for fail2ban"
  fi
  systemctl restart fail2ban  
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error restarting fail2ban"
  fi
  show_log info "installed and configured fail2ban"

  kill $PID
  /autosetup/blink.sh 0
}

function install_firewall {
  /autosetup/blink.sh 1 &
  PID="$!"

  ufw default deny incoming  
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error denying incoming traffic"
  fi
  ufw default allow outgoing  
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error allowing outgoing traffic"
  fi
  ufw allow ${SSH_PORT}  
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error allowing SSH Port"
  fi
  ufw enable  
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error enabling ufw"
  fi
  show_log info "installed and configured ufw"

  kill $PID
  /autosetup/blink.sh 0
}

function install_ros2 {
  /autosetup/blink.sh 1 &
  PID="$!"

  sudo curl -sSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.key -o /usr/share/keyrings/ros-archive-keyring.gpg
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error downloading rosdistro key"
  fi
  echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/ros-archive-keyring.gpg] http://packages.ros.org/ros2/ubuntu $(source /etc/os-release && echo $UBUNTU_CODENAME) main" | sudo tee /etc/apt/sources.list.d/ros2.list > /dev/null
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error setting up apt sources for ros"
  fi
  sudo apt-get update && sudo apt-get install -y ros-rolling-ros-base
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error installing ros rolling base"
  fi
  show_log info "installed and configured ros2 rolling base"

  kill $PID
  /autosetup/blink.sh 0
}

function install_rust {
  /autosetup/blink.sh 1 &
  PID="$!"

  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error installing rust"
  fi

#  sudo sh -c 'echo "deb [arch=amd64,arm64] http://repo.ros2.org/ubuntu/main `lsb_release -cs` main" > /etc/apt/sources.list.d/ros2-latest.list'
#  curl -s https://raw.githubusercontent.com/ros/rosdistro/master/ros.asc | sudo apt-key add -

  curl -s https://packagecloud.io/install/repositories/dirk-thomas/colcon/script.deb.sh | sudo bash
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error configuring colcon installation"
  fi
  sudo apt-get install -y python3-colcon-common-extensions
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error installing colcon"
  fi

  show_log info "installed and configured rust"
  kill $PID
  /autosetup/blink.sh 0
}

function clone_repo {
  git clone https://github.com/angelalonso/robot /home/$USER/
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error cloning the robot repository"
  fi
  show_log info "cloned robot repository"
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
  config_python
  install_fail2ban
  install_firewall
  install_ros2
  install_rust 
  clone_repo
  #connect_wifilan
}

## -------------- Main

run

