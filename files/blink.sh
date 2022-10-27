#!/usr/bin/env bash

## -------------- Vars

CONFIGFILE=".env"

## -------------- Step Functions

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


function blink_x_times {
  for j in $( eval echo {1..$1} )
  do
    echo "1" > /sys/class/gpio/gpio$LED_PIN/value
    if [ $? -ne 0 ]; then
      show_log warn "There was an issue writing to /sys/class/gpio/gpio$LED_PIN/value"
    fi
    sleep 0.2
    echo "0" > /sys/class/gpio/gpio$LED_PIN/value
    if [ $? -ne 0 ]; then
      show_log warn "There was an issue writing to /sys/class/gpio/gpio$LED_PIN/value"
    fi
    sleep 0.2
  done
}

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

load_dotenv

if [ $1 -gt 0 ]
then
  echo "$LED_PIN" | tee /sys/class/gpio/export &>/dev/null
  if [ $? -ne 0 ]; then
    show_log warn "There was an issue writing to /sys/class/gpio/export"
  fi

  echo "out" > /sys/class/gpio/gpio$LED_PIN/direction
  if [ $? -ne 0 ]; then
    show_log warn "There was an issue writing to /sys/class/gpio/gpio$LED_PIN/direction"
  fi

  while true
  do
    blink_x_times $1
    sleep 2
  done

  echo "0" > /sys/class/gpio/gpio$LED_PIN/value
  if [ $? -ne 0 ]; then
    show_log warn "There was an error writing to /sys/class/gpio/gpio$LED_PIN/value"
  fi

  echo "$LED_PIN" | tee /sys/class/gpio/unexport &>/dev/null
  if [ $? -ne 0 ]; then
    show_log warn "There was an issue writing to /sys/class/gpio/unexport"
  fi
else
  echo "0" > /sys/class/gpio/gpio$LED_PIN/value
  if [ $? -ne 0 ]; then
    show_log warn "There was an error writing to /sys/class/gpio/gpio$LED_PIN/value"
  fi

  echo "$LED_PIN" | tee /sys/class/gpio/unexport &>/dev/null
  if [ $? -ne 0 ]; then
    show_log warn "There was an issue writing to /sys/class/gpio/unexport"
  fi
fi
