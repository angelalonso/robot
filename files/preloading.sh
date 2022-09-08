#!/usr/bin/env bash

## -------------- Vars

CONFIGFILE=".env"

## -------------- Step Functions

function enable_ssh {
  BOOTPATH="$MICROSD_PATH/boot"
  if [ -d $BOOTPATH ]; then
    show_log info " - Enabling SSH"
    touch $BOOTPATH/ssh  
  else
    show_log err "$BOOTPATH not mounted, check MICROSD_PATH on $CONFIGFILE matches the actual mountpoint for both (e.g.: /media/user for both /media/user/rootfs and /media/user/boot)"
  fi
}

function configure_user {
  show_log info " - Configuring user and password hash"
  if [[ -z "$USER" ]] || [[ -z "$PASS" ]]; then
    show_log err "Either USER or PASS variables are empty! Check both variables on $CONFIGFILE"
  else
    if [ $(grep "^USER=" env.template ) == $(grep "^USER=" $CONFIGFILE) ] ||
       [ $(grep "^PASS=" env.template ) == $(grep "^PASS=" $CONFIGFILE) ]; then
      show_log err "USER or PASS are the same as in env.template! Check both variables on $CONFIGFILE and choose different values!"
    else
      PASSHASH=$(echo "$PASS" | openssl passwd -6 -stdin)
      echo $USER:$PASSHASH > ../files/userconf
    fi
  fi

}

function copy_files {
  BOOTPATH="$MICROSD_PATH/boot"
  ROOTPATH="$MICROSD_PATH/rootfs"
  if [ -d $BOOTPATH ] && [ -d $ROOTPATH ]; then
    show_log info " - Copying files to the MicroSD"
    sudo cp ../files/rc.local $ROOTPATH/etc/rc.local 
    sudo chmod +x $ROOTPATH/etc/rc.local 
    sudo mkdir -p $ROOTPATH/autosetup
    sudo cp ../files/autosetup.sh $ROOTPATH/autosetup/
    sudo chmod +x $ROOTPATH/autosetup/autosetup.sh
    sudo cp ../files/blink.sh $ROOTPATH/autosetup/
    sudo chmod +x $ROOTPATH/autosetup/blink.sh
    sudo cp ../files/.env $ROOTPATH/
    sudo cp ../files/userconf $BOOTPATH/
    if [ -f $SSHPUBPATH ]; then
      sudo cp $SSHPUBPATH $ROOTPATH/autosetup/sshpubkey
    else
      show_log err "$SSHPUBPATH public key not found!, check SSHPUBPATH on $CONFIGFILE"
    fi
    sudo cp ../files/sshd_config $ROOTPATH/autosetup/
  else
    show_log err "$BOOTPATH OR $ROOTPATH not mounted, check MICROSD_PATH on $CONFIGFILE matches the actual mountpoint for both (e.g.: /media/user for both /media/user/rootfs and /media/user/boot)"
  fi
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

function load_dotenv {
  if [ -f "$CONFIGFILE" ]; then
    export $(cat ${CONFIGFILE} | grep -v '^#' | xargs) >/dev/null
  else 
    show_log err ".env file does not exist. Have you created it from env.template?"
  fi
}

function remove_dotenv {
  # Avoid having the passwords around
  echo ""
  echo "Next we will be removing your user and password from the .env file, for security reasons..."
  echo ""
  echo "IMPORTANT: HAVE YOU SAVED THE PASSWORD SOMEWHERE? If not, do it now!"
  echo ""
  read -n 1 -s -r -p "Press any key to continue"
  sed -i '/^USER=/d' $CONFIGFILE
  sed -i '/^PASS=/d' $CONFIGFILE
  echo ""
}

function run {
  load_dotenv
  configure_user
  enable_ssh
  remove_dotenv
  copy_files
  show_log info " - ALL DONE! Now please insert the MicroSD on your Raspberry Pi and boot it up!"
}

## -------------- Main

run

