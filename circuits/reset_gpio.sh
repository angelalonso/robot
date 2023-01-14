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
for i in $(printenv | grep PIN | awk -F'=' '{print $2}' | sort | uniq); do
  echo $i >> /sys/class/gpio/unexport 2>/dev/null || true
done

