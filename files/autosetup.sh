#!/usr/bin/bash

# TODO:

# - auto restart
# - have a done.lock file to continue from last step

## -------------- Vars

CONFIGFILE="/.env"
LOCKFILE="/autosetup/autosetup.lock"

## -------------- Step Functions

function config_user {
  /autosetup/blink.sh $1 &
  PID="$!"

  mkdir -p /home/$NEWUSER/.ssh
  touch /home/$NEWUSER/.ssh/authorized_keys
  chown -R $NEWUSER /home/$NEWUSER

  cat /autosetup/sshpubkey | tee /home/$NEWUSER/.ssh/authorized_keys
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error adding pubkey"
  fi

  sudo chmod 600 /home/$NEWUSER/.ssh/authorized_keys  
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error changing access to authorized_keys"
  fi

  sudo chown -R $NEWUSER:$NEWUSER /home/$NEWUSER 
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error changing owner to /home/$NEWUSER"
  fi

  # TODO: this wasnt tested
  sudo usermod -aG kmem,dialout $NEWUSER
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error adding $NEWUSER to kmem group"
  fi
  show_log info "adding SSH key done"

  kill $PID
  /autosetup/blink.sh 0
}

function secure_ssh {
  /autosetup/blink.sh $1 &
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


function config_python {
  /autosetup/blink.sh $1 &
  PID="$!"

  python3 -m pip install --upgrade pip
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error upgrading pip"
  fi
  pip install flatdict maturin python-dotenv flask RPi.GPIO
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
  /autosetup/blink.sh $1 &
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
  /autosetup/blink.sh $1 &
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
  ufw allow ${SSHPORT}  
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error allowing SSH Port"
  fi
  ufw --force enable  
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
  /autosetup/blink.sh $1 &
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
  sudo apt-get  -o DPkg::Lock::Timeout=300 update && sudo apt-get install -y ros-rolling-ros-base
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
  /autosetup/blink.sh $1 &
  PID="$!"

  # TODO: this wasnt tested
  sudo -i -u $NEWUSER bash << EOF
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
EOF
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error installing rust for user $NEWUSER"
  fi

  curl -s https://packagecloud.io/install/repositories/dirk-thomas/colcon/script.deb.sh | sudo bash
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error configuring colcon installation"
  fi
  sudo apt-get -o DPkg::Lock::Timeout=300 install -y python3-colcon-common-extensions
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
  /autosetup/blink.sh $1 &
  PID="$!"

  # Adding resilience for the fall that it fails half-way and we restart
  if [[ -d "/home/$NEWUSER/robot" ]]; then
    if [[ -d "/home/$NEWUSER/robot/brain" ]]; then
      runuser -l $NEWUSER -c "cd /home/$NEWUSER/robot; git pull"
    else
      rm -rf /home/$NEWUSER/robot
      #TODO: needed? runuser -l $NEWUSER -c 'git config --global protocol.version 1'
      runuser -l $NEWUSER -c "git clone https://github.com/angelalonso/robot /home/$NEWUSER/robot"
    fi
  else
    #:runuser -l $NEWUSER -c 'git config --global protocol.version 1'
    runuser -l $NEWUSER -c "git clone https://github.com/angelalonso/robot /home/$NEWUSER/robot"
  fi
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error cloning the robot repository"
  fi

  # TODO: Not needed?
  #chown $NEWUSER:$NEWUSER -R /home/$NEWUSER/robot
  #if [ $? -ne 0 ]; then
  #  kill $PID
  #  /autosetup/blink.sh 0
  #  show_log err "There was an error making $NEWUSER owner of the cloned repository folder"
  #fi

  runuser -l $NEWUSER -c "cp /.env /home/$NEWUSER/robot/brain/.env"
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error copying the .env to the actual program"
  fi

  runuser -l $NEWUSER -c "cd /home/$NEWUSER/robot/brain && ./run.sh build"
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error building the code"
  fi

  show_log info "cloned robot repository and built it"
  kill $PID
  /autosetup/blink.sh 0
}


function prepare_rclocal {
  /autosetup/blink.sh $1 &
  PID="$!"

  echo
  systemctl enable rc-local
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error enabling rc-local"
  fi

  show_log info "enabled rc-local"
  kill $PID
  /autosetup/blink.sh 0
}

function prepare_robotrun {
  /autosetup/blink.sh $1 &
  PID="$!"

  # TODO: compile current code

  echo
  systemctl enable robot
  if [ $? -ne 0 ]; then
    kill $PID
    /autosetup/blink.sh 0
    show_log err "There was an error enabling robot"
  fi

  show_log info "enabled robot"
  kill $PID
  /autosetup/blink.sh 0

}

## -------------- Aux Functions

function show_log {
  TSTAMP=$(date "+%Y-%m-%d %H:%M:%S")
  case $1 in
    "debug"|"d")
      echo "[$TSTAMP] --- [DEBUG] ---" >> $LOGFILE
      echo "[$TSTAMP][DEBUG] - $2" >> $LOGFILE
      echo "[$TSTAMP][DEBUG] - $2"
      ;;
    "info"|"i")
      echo "[$TSTAMP] --- [INFO] ---"
      echo "[$TSTAMP][INFO] - $2" >> $LOGFILE
      echo "[$TSTAMP][INFO] - $2"
      ;;
    "warn"|"w")
      echo "[$TSTAMP] --- [WARN] ---"
      echo "[$TSTAMP][WARN] - $2" >> $LOGFILE
      echo "[$TSTAMP][WARN] - $2"
      ;;
    "error"|"err"|"e")
      echo "[$TSTAMP] --- [ERROR] ---"
      echo "[$TSTAMP][ERROR] - $2" >> $LOGFILE
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
  LOGFILE="autosetup-extra.log"
  touch $LOGFILE
  if [ ! -f ${LOCKFILE} ]; then
    load_dotenv 11
    config_user 10
    secure_ssh 9
    config_python 8
    install_fail2ban 7
    install_firewall 6
    install_ros2 5
    install_rust 4
    clone_repo 3
    prepare_rclocal 2
    prepare_robotrun 1
  else
    echo "Configuration ran properly. Skipping..."
  fi

  touch ${LOCKFILE}

}

## -------------- Main

run

