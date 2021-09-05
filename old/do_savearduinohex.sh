#!/usr/bin/env bash

die () {
    echo >&2 "$@"
    exit 1
}

[ "$#" -eq 1 ] || die "ERROR: please provide the name of the arduino program (without the .ino.hex), $# provided"

ARDUFILE=$1
TMPFLDR=""

for i in $(ls -d /tmp/* | grep arduino_build); do 
  FILE=$(find $i -name "${ARDUFILE}.ino.hex")
  if [[ ${FILE} != "" ]]; then
    echo "copying over "${FILE}
    sudo cp ${FILE} ./arduino/${ARDUFILE}/
    sudo chown -R ${USER}. ./arduino
  else
    echo "nothing found!"
  fi
done
