#!/usr/bin/env zsh

function read_dotenv() {
  while read line
  do
    TYPE=${line:0:3} 
    if [[ "${TYPE}" == "LED" ]]
    then
      LEDNR=$(echo $line | awk -F '=' '{print $2}')
      LEDPINS+=("${LEDNR}")
    fi
  done < .env
}

function show_ledpins() {
  for pin in "${LEDPINS[@]}"
  do
    echo " - $pin"
  done


}

LEDPINS=()

read_dotenv
show_ledpins
