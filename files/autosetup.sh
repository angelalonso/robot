#!/usr/bin/env bash

## -------------- Vars

CONFIGFILE=".env"

## -------------- Step Functions

## -------------- Aux Functions

function blink_x_times {
  for j in $( eval echo {1..$1} )
  do
    echo "1" > /sys/class/gpio/gpio$LED_PIN/value
    sleep 0.2
    echo "0" > /sys/class/gpio/gpio$LED_PIN/value
    sleep 0.2
  done
}

function blink {
  if [ -d /sys/class/gpio/gpio$LED_PIN/ ]; then
    echo "$LED_PIN" > /sys/class/gpio/export  
    echo "out" > /sys/class/gpio/gpio$LED_PIN/direction

    blink_x_times 20

    echo "0" > /sys/class/gpio/gpio$LED_PIN/value
  else
    show_log err "GPIO $LED_PIN NOT FOUND!"
  fi
}

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
  blink
}

## -------------- Main

run

