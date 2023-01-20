#!/usr/bin/env bash

CONFIGFILE=".env"

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

NR_OF_RUNS=24
SLEEPTIME=0.01
for i in $(seq ${NR_OF_RUNS})   # you can also use {0..9}
do
  echo "---- Run Nr. ${i} ----"
  for i in /do/led_switch /do/led_switch /do/led_switch /do/led_switch /do/led_switch /do/led_switch /do/led_switch /do/led_switch
  #for i in /get/mode /do/fwd /do/bwd /do/left /do/right /do/stop /do/led_switch /do/led_switch /do/led_switch /do/led_switch /do/led_switch /do/led_switch /do/led_switch /do/led_switch
  do
    echo Calling $i
    #RESULT=$(curl --header "Content-Type: application/json" --request POST http://${SSHIP}:${APIPORT}${i})
    RESULT=$(curl -w "\n\n%{time_connect} + %{time_starttransfer} = %{time_total}\n" --header "Content-Type: application/json" --request POST http://${SSHIP}:${APIPORT}${i})
    echo $?
    read -p "Continuing in "${SLEEPTIME}" Seconds...." -t ${SLEEPTIME}
    echo
  done
done

