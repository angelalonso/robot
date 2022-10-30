#!/usr/bin/bash

##  Troubleshoot?  as root:
## sudo cloud-init init --local
## sudo cloud-init init
## sudo cloud-init schema --system
## sudo cloud-init modules --mode=config
## sudo cloud-init modules --mode=final
## 
## rm /var/lib/cloud/instance/sem/* && sudo cloud-init modules --mode=config && sudo cloud-init modules --mode=final
# TODO: asks to restart plymouth on ros2 and rust install, why?


## -------------- Vars

CONFIGFILE=".env"

## -------------- Step Functions

function load_dotenv {
  if [ -f "$CONFIGFILE" ]; then
    if [ $(grep PASS $CONFIGFILE | wc -l) -eq 0 ]; then
      show_log err ".env file does not have the PASS variable, please add it! (HINT: are you reusing the file from a previous run?)"
    else
      #export $(cat ${CONFIGFILE} | grep -v '^#' | xargs) >/dev/null
      #export $(cat ${CONFIGFILE} | grep -v '^#' | sed -e '/^#/d;/^\s*$/d' -e "s/'/'\\\''/g" -e "s/=\(.*\)/='\1'/g" | xargs) >/dev/null
      set -o allexport
      source .env
      set +o allexport
    fi
  else 
    show_log err ".env file does not exist. Have you created it from env.template?"
  fi
}

function modify_files {
  cp user-data.template user-data
  PASSWD=$(echo '${PASS}' | mkpasswd -m sha-512 -s)
  sed -i -e "s/    passwd: AAA/    passwd: ${PASSWD//\//\\\/}/g" user-data
  AUTHKEY=$(cat $SSHPUBPATH)
  sed -i -e "s/     - SSHAUTHKEY_TOBEREPLACED/     - ${AUTHKEY//\//\\\/}/g" user-data

  cp sshd_config.template sshd_config
  sed -i -e "s/#Port 22/Port ${SSHPORT}/g" sshd_config

  cp 50-cloud-init.yaml.template 50-cloud-init.yaml
  sed -i -e "s/\"WIFISSID\"/\"${WIFI_SSID}\"/g" 50-cloud-init.yaml
  sed -i -e "s/\"WIFIPASS\"/\"${WIFI_PASS}\"/g" 50-cloud-init.yaml

  # TODO: needed?
  cp wpa_supplicant.conf.template wpa_supplicant.conf
  sed -i -e "s/  ssid=\"\"/  ssid=\"${WIFI_SSID}\"/g" wpa_supplicant.conf
  sed -i -e "s/  psk=\"\"/  psk=\"${WIFI_PASS}\"/g" wpa_supplicant.conf

  # TODO: this does not substitute NEWUSER
  cp robot.service.template robot.service
  sed -i -e "s/NEWUSER/${NEWUSER}/g" robot.service
}

function copy_files {
  BOOTPATH="$MICROSD_PATH/system-boot"
  ROOTPATH="$MICROSD_PATH/writable"
  if [ -d $BOOTPATH ] && [ -d $ROOTPATH ]; then
    show_log info " - Copying files to the MicroSD"
    sudo mkdir -p $ROOTPATH/autosetup
    sudo cp autosetup.sh $ROOTPATH/autosetup/
    sudo cp blink.sh $ROOTPATH/autosetup/
    sudo cp .env $ROOTPATH/
    sudo cp user-data $BOOTPATH/
    sudo cp sshd_config $ROOTPATH/autosetup/
    sudo cp 50-cloud-init.yaml $ROOTPATH/etc/netplan/50-cloud-init.yaml
    sudo cp 99-disable-network-config.cfg $ROOTPATH/etc/cloud/cloud.cfg.d/99-disable-network-config.cfg 
  # TODO: needed?
    sudo cp wpa_supplicant.conf $ROOTPATH/etc/wpa_supplicant/wpa_supplicant.conf
    sudo cp rc.local $ROOTPATH/etc/rc.local
    sudo cp rc-local.service $ROOTPATH/lib/systemd/system/rc-local.service
    sudo cp robot.service $ROOTPATH/lib/systemd/system/robot.service
    sudo chmod +x $ROOTPATH/autosetup/autosetup.sh
    sudo chmod +x $ROOTPATH/autosetup/blink.sh
    sudo chmod +x $ROOTPATH/etc/rc.local
    # TODO: needed?
    if [ -f $SSHPUBPATH ]; then
      ssh-keygen -l -f $SSHPUBPATH
      if [ $? -eq 0 ]; then 
        sudo cp $SSHPUBPATH $ROOTPATH/autosetup/sshpubkey
      else
        show_log err "$SSHPUBPATH public key is not correct! check that it is indeed a proper public key"
      fi
    else
      show_log err "$SSHPUBPATH public key not found!, check SSHPUBPATH on $CONFIGFILE"
    fi
  else
    show_log err "$BOOTPATH OR $ROOTPATH not mounted, check MICROSD_PATH on $CONFIGFILE matches the actual mountpoint for both (e.g.: /media/user for both /media/user/rootfs and /media/user/boot)"
  fi
}

function clean_up {
  rm user-data
  rm sshd_config
  rm 50-cloud-init.yaml
  rm wpa_supplicant.conf
}

function remove_PASS_from_dotenv {
  # Avoid having the passwords around
  echo ""
  echo "Next we will be removing your user's password from the .env file, for security reasons..."
  echo ""
  echo "IMPORTANT: HAVE YOU SAVED THE PASSWORD SOMEWHERE? If not, do it now!"
  echo ""
  read -n 1 -s -r -p "Press any key to continue"
  sed -i '/^PASS=/d' $CONFIGFILE
  echo ""
}

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

function run {
  load_dotenv
  modify_files
  copy_files
  remove_PASS_from_dotenv
  clean_up
  sudo umount $BOOTPATH
  sudo umount $ROOTPATH

  show_log info " - MicroSD READY! Now please do as follows:"
  show_log info "  * Insert the MicroSD on your Raspberry Pi and prepare to boot your robot up"
  show_log info "    * Make sure it is connected to your wifi router using RJ45 - IMPORTANT! if you don't you will have to restart the whole process!"
  show_log info "    * Connect to a power outlet and not the battery"
  show_log info "  * Wait until it starts blinking (can take up to 15 minutes)"
  show_log info "  * Then wait another ~30 minutes until the final configuration is done"
  show_log info "   . The number of blinks shows the number of steps remaining until robot is ready"
  show_log info "   . At one point you will already be able to SSH into the machine"
}

## -------------- Main

run

