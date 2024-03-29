#!/usr/bin/env bash


set -eo pipefail
shopt -s extglob # required for proper string substitution

# AUX FUNCTIONS
##################

function is_robot_available() {
  ## Here we check that the host is available
  READY=false
  OK=false
  SLEEPTIME=1
  while ! ${READY}; do
    ## && RES=$? is needed because ping did not return an exit code I can catch on this script
    ping -c 1 ${SSHIP} >/dev/null && RES=$?
    #ping -c 1 google.com >/dev/null && RES=$?
    AUX=$?
    if [[ ${RES} == "" ]]; then RES=${AUX}; fi

    if [[ ${RES} != 0 ]]; then
      show_log w " Host ${SSHIP} unreachable from this machine"
      sleep ${SLEEPTIME}
      SLEEPTIME=$((SLEEPTIME * 2)) # "increasing wait until retry", and a limit of 4 seconds
      if [[ ${SLEEPTIME} -gt 4 ]]; then
        READY=true
      fi
    else
      READY=true
      OK=true
    fi
  done
  if [[ ${OK} == false ]]; then
    show_log e " Host ${SSHIP} is not available!"
  else
    show_log i " Host ${SSHIP} available."
  fi
}

function clean_gpio() {
  echo "Unexporting GPIO PINs..."
  sudo ${CODEPATH}/reset_gpio.sh || true
  echo "...Done!"
}

function kill_switch() {
  echo "Killing leftovers of circuits..."
  for pid in $(ps aux | grep circuits | grep -v vim | awk '{print $2}'); do
    sudo kill -9 $pid || true
  done
  echo "...Killed!"
  clean_gpio
}

function ctrl_c() {
  # once the launch run is stopped with CTRL-C we want to clean up
  echo "** Trapped CTRL-C"

  for i in $(ps aux | grep target | grep circuits | awk '{print $2}'); do echo $i;kill $i;done
  kill_switch
  # TODO: maybe have different ctrl_c for in-robot and local

  cd ${CWDMAIN}
  exit 2
}

function show_log {
  TSTAMP=$(date "+%Y-%m-%d %H:%M:%S")
  case $1 in
    "debug"|"d")
      printf '\e[35m%s\n\e[0m' "[$TSTAMP][DEBUG] - $2"
      ;;
    "info"|"i")
      printf '\e[32m%s\n\e[0m' "[$TSTAMP][INFO] - $2"
      ;;
    "warn"|"w")
      printf '\e[33m%s\n\e[0m' "[$TSTAMP][WARN] - $2"
      ;;
    "error"|"err"|"e")
      printf '\e[31m%s\n\e[0m' "[$TSTAMP][ERROR] - $2"
      exit 2
      ;;
    *)
      printf '\e[35m%s\n\e[0m' "[$TSTAMP][DEBUG] - $2"
      ;;
  esac
}

function load_dotenv {
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

function do_mode() {
  if [[ "$1" == "help" ]]; then
    show_help
  elif [[ "$1" == "test" ]]; then
    dev_test
  elif [[ "$1" == "test_arduino" ]]; then
    dev_test_arduino
  elif [[ "$1" == "deploy" ]]; then
    is_robot_available
    trigger_deploy
  elif [[ "$1" == "run" ]]; then
    is_robot_available
    trigger_run
  elif [[ "$1" == "live_run" ]]; then
    live_run
  elif [[ "$1" == "push" ]]; then
    trigger_push
  elif [[ "$1" == "kill" ]]; then
    dev_kill
  elif [[ "$1" == "live_kill" ]]; then
    live_kill
  elif [[ "$1" == "" ]]; then
    dev_test 
    is_robot_available
    trigger_deploy
    trigger_run
  else
    show_help
  fi
}

function show_help() {
  show_log i "SYNTAX:"
  show_log i " $0        - Test locally, then Deploy, Build, and Run on robot"
  show_log i " - Main options:"
  show_log i " $0 test - "
  show_log i " $0 test_arduino - "
  show_log i " $0 deploy    - "
  show_log i " $0 run       - "
  show_log i " $0 live_run       - "
  show_log i " $0 push       - "
  show_log i " $0 kill       - "
  show_log i " $0 live_kill       - "
  show_log i " - Other options:"
  show_log i " $0 help      - Show this help"
}


# ACTION FUNCTIONS
####################


function dev_test() {
  show_log i "##################  LOCAL TEST  ####################"
  trap ctrl_c INT

  cd ${CODEPATH}
  cargo check
  if [[ $? != 0 ]]; then
    show_log e "- Cargo Check: Errors found"
  else
    show_log i "- Cargo Check: Everything OK"
  fi

  cargo clippy
  if [[ $? != 0 ]]; then
    show_log e "- Cargo Clippy: Errors found"
  else
    show_log i "- Cargo Clippy: Everything OK"
  fi

  cargo test
  if [[ $? != 0 ]]; then
    show_log e "- Cargo Test: Errors found"
  else
    show_log i "- Cargo Test: Everything OK"
  fi

  #cargo run ## Let's play a game: this is forbidden in the local dev environment!
}

function dev_test_arduino() {
  show_log i "##################  LOCAL TEST OF ARDUINO  #########"
  show_log d "              ####  PLEASE CONNECT ARDUINO DIRECTLY TO YOUR LAPTOP"
  trap ctrl_c INT

  cd ${CODEPATH}
  cargo test -- --ignored --show-output --nocapture  
  if [[ $? != 0 ]]; then
    show_log e "- Cargo Check: Errors found"
  else
    show_log i "- Cargo Check: Everything OK"
  fi

}


function trigger_push() {
  show_log d "              ####  PUSHING CHANGES TO GIT #########"
  trap ctrl_c INT

  cd ${CWDMAIN}
  cd ${ROBOTLIB}
  git commit -am "robot.sh: automatically committing latest 'working' version" || true
  git push origin ${GIT_BRANCH}
  
}

function trigger_deploy() {
  show_log i "##################  DEPLOYING AT ROBOT  ############"
  trap ctrl_c INT

  trigger_push
  
  show_log d "              ####  COMPILING ON ROBOT  #########"
  ssh ${NEWUSER}@${SSHIP} -p${SSHPORT} "cd \$HOME/robot && git pull && cd ${ROBOTLIB} && /home/robotadm/.cargo/bin/cargo build --release"
}

function trigger_run() {
  show_log i "##################  CALLING RUN ON ROBOT  ##########"
  trap ctrl_c INT

  ssh ${NEWUSER}@${SSHIP} -p${SSHPORT} "cd \$HOME/robot && ./robot.sh live_kill || true && ./robot.sh live_run"
}

function live_run() {
  show_log i "##################  RUNNING ON ROBOT  ##############"
  trap ctrl_c INT

  clean_gpio
  cd ${CODEPATH} && sudo ./target/release/circuits
}


function dev_kill() {
  show_log i "##################  RESETTING EVERYTHING ON ROBOT  ##############"
  trap ctrl_c INT

  ssh ${NEWUSER}@${SSHIP} -p${SSHPORT} "cd \$HOME/robot && ./robot.sh live_kill"
}

function live_kill() {
  show_log i "##################  RESETTING EVERYTHING FROM ROBOT SIDE  ##############"
  trap ctrl_c INT

  kill_switch
}

# MAIN
####################

CWDMAIN=$(pwd)
ROBOTLIB="circuits"
CONFIGFILE="${CWDMAIN}/.env"
CODEPATH="${CWDMAIN}/${ROBOTLIB}"

load_dotenv
do_mode $@
