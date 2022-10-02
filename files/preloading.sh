#!/usr/bin/env bash

# TODO:

## -------------- Vars

CONFIGFILE=".env"

## -------------- Step Functions

function load_dotenv {
  if [ -f "$CONFIGFILE" ]; then
    if [ $(grep PASS $CONFIGFILE | wc -l) -eq 0 ]; then
      show_log err ".env file does not have the PASS variable, please add it! (HINT: are you reusing the file from a previous run?)"
    else
      export $(cat ${CONFIGFILE} | grep -v '^#' | xargs) >/dev/null
    fi
  else 
    show_log err ".env file does not exist. Have you created it from env.template?"
  fi
}

function modify_files {
  cp sshd_config sshd_config.orig
  sed -i -e "s/#Port 22/Port ${SSHPORT}/g" sshd_config

  cp wpa_supplicant.conf wpa_supplicant.conf.orig
  sed -i -e "s/ssid=\"\"/ssid=\"${WIFI_SSID}\"/g" wpa_supplicant.conf
  sed -i -e "s/psk=\"\"/psk=\"${WIFI_PASS}\"/g" wpa_supplicant.conf
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
    sudo cp wpa_supplicant.conf $ROOTPATH/etc/wpa_supplicant/wpa_supplicant.conf
    sudo mv sshd_config.orig sshd_config # restore original
    sudo chmod +x $ROOTPATH/autosetup/autosetup.sh
    sudo chmod +x $ROOTPATH/autosetup/blink.sh
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
#
  sudo umount $BOOTPATH
  sudo umount $ROOTPATH

  show_log info " - MicroSD READY! Now please do as follows:""insert the MicroSD on your Raspberry Pi and boot it up!"
  show_log info "  * Insert the MicroSD on your Raspberry Pi and boot it up"
  show_log info "  * Wait until it starts blinking (can take up to 15 minutes)"
  show_log info "  * Then wait another ~30 minutes until the final configuration is done"
  show_log info "   . The number of blinks shows the number of steps remaining until robot is ready"
  show_log info "   . At one point you will already be able to SSH into the machine"
}

## -------------- Main

run

