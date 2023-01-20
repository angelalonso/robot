#!/usr/bin/env bash
#

set -eo pipefail

function step_1_burn() {
  show_log info "(STEP 1:) Before running this script you should have:"
  show_log info "  Downloaded Raspberry Pi OS Armhf Lite image from https://downloads.raspberrypi.org/raspios_lite_armhf/images/"
  show_log info "  Burnt it into the microSD (Using Balena etcher for instance)"
}

function step_2_user() {
  show_log info "STEP 2: configuring user"
  # TODO:
  # Mount 'boot'
  # openssl passwd -6 # encode the new password
  # vim /media/$USER/boot/userconf.txt # add user:encoded_pass and save
  BOOTPATH="$MICROSD_PATH/boot"
  ROOTPATH="$MICROSD_PATH/rootfs"
  if [ -d $BOOTPATH ] && [ -d $ROOTPATH ]; then
    PASSWD=$(echo '${PASS}' | openssl passwd -6 -stdin)
    echo "${NEWUSER}:${PASSWD}" > ${BOOTPATH}/userconf.txt
    show_log info "  DONE!"
  else
    show_log err "$BOOTPATH OR $ROOTPATH not mounted, check MICROSD_PATH on $CONFIGFILE matches the actual mountpoint for both (e.g.: /media/user for both /media/user/rootfs and /media/user/boot)"
  fi
}

function step_3_more() {
  show_log info "(STEP 3:) Now do the following:"

  echo "- Unmount the MicroSD card"
  echo "- Insert it to the raspberry pi"
  echo "- Connect it to power and the network (use a cable for now, mate!)"
  echo "- Find the IP with $ nmap -sP 192.168.0.0/24"  

  show_log info "(STEP 4:) SSH into it, give it a name, tweak raspbian"

  echo " $ ssh <USER>@\<IP\> # password is whatever you chose previously"
  echo " - Once in, change hostname to your liking"
  echo " $ sudo hostname \$HOSTNAME"
  echo " $ sudo vim /etc/hosts # change raspberrypi to \$HOSTNAME"
  echo " $ sudo vim /etc/hostname # change raspberrypi to \$HOSTNAME"
  echo " - Tweak raspbian"
  echo " $ sudo raspi-config"
  echo " > Localisation Options > Change Locale > choose the Locales you need, hit OK"
  echo " > Exit > Reboot"
 
  show_log info "(STEP 5:) Update, upgrade, install the basics"
  echo " $ sudo apt-get update"
  echo " $ sudo apt-get upgrade"
  echo " $ sudo apt-get install -y pigpio vim git libudev-dev"
  echo " - Make pigpio daemon NOT run at boot"
  echo " $ sudo systemctl disable pigpiod"
  echo " - Configure your own user"
  echo " - Add your SSH key to log in"
  echo " $ sudo mkdir -p /home/$NEWUSER/.ssh"
  echo " $ sudo vi /home/$NEWUSER/.ssh/authorized_keys # Here you should paste your public SSH key and save"
  echo " $ sudo chmod 600 /home/$NEWUSER/.ssh/authorized_keys"
  echo " $ sudo chown -R $NEWUSER:$NEWUSER /home/$NEWUSER"
  echo " - Log out, log in again"
  echo " $ logout"
  echo " $ ssh -i <PATH TO YOUR SSH KEY> $NEWUSER@<IP>"
  echo " $ cd && git clone https://github.com/angelalonso/robot"
  echo " - Get in the robot folder and edit .env"
  echo " - Copy over the contents of your local .env"
   
  show_log info "(STEP 6:) Strengthen SSH"
  echo " $ sudo vim /etc/ssh/sshd_config"
  echo ""
  echo " ChallengeResponseAuthentication no"
  echo " PasswordAuthentication no"
  echo " UsePAM no"
  echo " PermitRootLogin no"
  echo " Port $NEWPORT"
  echo ""
  echo " $ sudo systemctl reload ssh"
  echo " - Install fail2ban"
  echo " $ sudo apt-get update && sudo apt-get install -y fail2ban"
  echo " $ sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local"
  echo " $ sudo systemctl restart fail2ban"

  show_log info "(STEP 7:) Strengthen SSH"
  echo " - Install and configure UFW"
  echo " $ sudo apt-get update && sudo apt-get install -y ufw"
  echo " - Configure default config"
  echo " $ sudo ufw default deny incoming"
  echo " $ sudo ufw default allow outgoing"
  echo " $ sudo ufw allow ${SSH_PORT}"
  echo " $ sudo ufw enable"

  show_log info "(STEP 7:) Install Rust"
  echo " $ curl https://sh.rustup.rs -sSf | sh"

  show_log info "(STEP 7:) Make Raspberry connect to LAN through Wi-Fi"
  echo " - Connect your Wifi dongle."
  echo " - Read https://www.raspberrypi.org/documentation/configuration/wireless/wireless-cli.md"
  echo " $ sudo raspi-config"
  echo " > Localisation Options > Change Wi-fi Country > Choose yours"
  echo " > Network Options > Wi-Fi > add the name of the WiFi network and the pass"

}

### - AUX Functions

function show_log {
  TSTAMP=$(date "+%Y-%m-%d %H:%M:%S")
  case $1 in
    "debug"|"d")
      #echo "[$TSTAMP][DEBUG] - $2"
      printf '\e[35m%s\n\e[0m' "[$TSTAMP][DEBUG] - $2"
      ;;
    "info"|"i")
      #echo "[$TSTAMP][INFO] - $2"
      printf '\e[32m%s\n\e[0m' "[$TSTAMP][INFO] - $2"
      ;;
    "warn"|"w")
      #echo "[$TSTAMP][WARN] - $2"
      printf '\e[33m%s\n\e[0m' "[$TSTAMP][WARN] - $2"
      ;;
    "error"|"err"|"e")
      #echo "[$TSTAMP][ERROR] - $2"
      printf '\e[31m%s\n\e[0m' "[$TSTAMP][ERROR] - $2"
      ;;
    *)
      #echo "[$TSTAMP][DEBUG] - Wrong Logging mode"
      printf '\e[35m%s\n\e[0m' "[$TSTAMP][DEBUG] - $2"
      exit 2
      ;;
  esac
}

function load_dotenv {
  # TODO:
  # if .env does not exist, copy from env.template, open for edit
  # - load again
  # TODO: offer a way to modify .env later
  if [ -f "$CONFIGFILE" ]; then
    if [ $(grep PASS $CONFIGFILE | wc -l) -eq 0 ]; then
      show_log err ".env file does not have the PASS variable, please add it! (HINT: are you reusing the file from a previous run?)"
    else
      set -o allexport
      source .env
      set +o allexport
    fi
  else 
    show_log err ".env file does not exist. Have you created it from env.template?"
  fi
}


### - Main

CONFIGFILE=".env"
load_dotenv
step_1_burn
step_2_user
step_3_more
