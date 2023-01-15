#!/usr/bin/env bash
#
#   - Build -> Test -> Crossbuild -> Deploy -> Run
#   The point is that it should progress based on checks:
#   - Am I running on the Raspberry machine?
#   - Is the Robot available through SSH?
#   - Did all steps finish properly?
#   TODO: 
#   clippy and tests on build
#   check ports and gpio before run
#   localbuild, localrun, build as localbuild on the robot and same for run

set -eo pipefail
shopt -s extglob # required for proper string substitution


# ACTION FUNCTIONS

function do_build() {
  show_log i "################## BUILD on ${ARCH} ####################"
  trap ctrl_c INT

  cd ${CODEPATH}
  cargo update && cargo build --release
  if [[ $? -ne 0 ]]; then
    show_log w "Nothing was built"
  fi

  cd ${CWDMAIN}
}

function do_crossbuild() {
  show_log i "################## CROSS BUILD for AARCH64 ####################"
  trap ctrl_c INT

  # Build only if anything changed (comparing to git), 
  # TODO: we need to define this "git control" better (what if it was already commited?)
  #       One error is: as long as you dont commit, you can build without being asked, even if you didnt change since latest commit
  #BUILDORNOT=false
  #if [[ $(git status -s ${CODEPATH} | wc -l) -gt 0 ]]; then
  #  BUILDORNOT=true
  #else
  #  show_log w "There seems to be no changes on ${CODEPATH}"
  #  show_log w "Do you still want to Build? (just press y or n)"
  #  LOOP=true
  #  while [[ $LOOP == true ]] ; do
  #    read -r -n 1 -p "[y/n]: " REPLY
  #    case $REPLY in
  #      [yY])
  #        echo
  #        BUILDORNOT=true
  #        LOOP=false
  #        ;;
  #      [nN])
  #        echo
  #        BUILDORNOT=false
  #        LOOP=false
  #        ;;
  #      *) echo;show_log w "Invalid Input, please answer y or n. Do you want to Build?"
  #    esac
  #  done
  #fi

  #if [[ "${BUILDORNOT}" == true ]]; then
  #  cargo update
  #  cd ${CODEPATH}
  #  cargo build --target aarch64-unknown-linux-gnu

  #  #TODO: check if it worked
  #else
  #  show_log w "Nothing was built"
  #fi
  #cd $CWDMAIN

  # uncomment this to get the image!
  docker build \
    --build-arg NEWUSER=${NEWUSER} \
    --platform linux/arm64/v8 \
    -t aarch64-cross .

  #cd ${CWDMAIN}
  docker run \
    --rm \
    --user ${NEWUSER} \
    --platform linux/arm64/v8 \
    -v $PWD:/home/${NEWUSER}/robot \
    -e "ARCH=aarch64" \
    -it aarch64-cross \
    /bin/bash -c "source /home/${NEWUSER}/.cargo/env && cd robot && ./robot.sh build"

  # TODO: add some tests
}


function do_run() {
  show_log i "##################  RUN  ####################"
  trap ctrl_c INT

  # TODO: modify this when new PINS are in use
  echo "${LED_PIN}" | sudo tee /sys/class/gpio/unexport >/dev/null 2 || true 
  echo "${MOTOR_R_PIN_IN1}" | sudo tee /sys/class/gpio/unexport >/dev/null || true 
  echo "${MOTOR_R_PIN_IN2}" | sudo tee /sys/class/gpio/unexport >/dev/null || true 
  echo "${MOTOR_R_PIN_ENA}" | sudo tee /sys/class/gpio/unexport >/dev/null || true 
  echo "${MOTOR_L_PIN_IN1}" | sudo tee /sys/class/gpio/unexport >/dev/null || true 
  echo "${MOTOR_L_PIN_IN2}" | sudo tee /sys/class/gpio/unexport >/dev/null || true 
  echo "${MOTOR_L_PIN_ENA}" | sudo tee /sys/class/gpio/unexport >/dev/null || true 
  show_log i "(please ignore errors above about unexport)"

  cd ${CODEPATH}
  RUNPATH=${CODEPATH}/target/release/circuits
  sudo ${RUNPATH}
}

function do_crossrun() {
  show_log i "##################  RUN  ####################"
  trap ctrl_c INT

  ## Prepare GPIO before running
  #show_log i "Initializing GPIOs"
  #echo ${LEDMAIN_PIN} | sudo tee /sys/class/gpio/export >/dev/null
  #echo "out" | sudo tee /sys/class/gpio/gpio${LEDMAIN_PIN}/direction >/dev/null
  #echo 1 | sudo tee /sys/class/gpio/gpio${LEDMAIN_PIN}/value >/dev/null
  #echo 0 | sudo tee /sys/class/gpio/gpio${LEDMAIN_PIN}/value >/dev/null

  cd ${CODEPATH}
  cargo run 
}

#function do_clean() {
#  # Keep the two latest folders, compress the third latest, remove the rest
#  show_log i "##################  CLEANUP OLD BUILDS  ####################"
#  trap ctrl_c INT
#
#  cd ${CODEPATH}
#  for i in ${ROS_PCKGS[@]}; do
#    show_log i "Cleaning up ${i}"
#    cd src/${i}
#    # STEP 1: Leave only the latest 3 directories
#    RETAIN_LATEST=3
#    ix=0
#    set +e
#    for k in $(ls -d ${CODEPATH}/src/${i}/versions/*/ 2>/dev/null | sort -r ); do
#      set -e
#      if [[ $ix -ge $RETAIN_LATEST ]]; then
#        show_log d "Removing old version ${k}"
#        rm -r ${k}
#      fi
#      ix=$((ix+1))
#    done
#    set -e
#    # STEP 2: Compress the third latest directory
#    RETAIN_LATEST=2
#    ix=0
#    set +e
#    for k in $(ls -d ${CODEPATH}/src/${i}/versions/*/ 2>/dev/null | sort -r ); do
#      set -e
#      if [[ $ix -ge $RETAIN_LATEST ]]; then
#        show_log d "Compressing ${k::-1}"
#        tar -zcf ${k::-1}.tar.gz ${k::-1} 2>/dev/null
#        show_log d "Cleaning up original folder ${k::-1}"
#        rm -r ${k::-1}
#      fi
#      ix=$((ix+1))
#    done
#    set -e
#    # STEP 3: Make sure we only keep that compressed file
#    RETAIN_LATEST=1
#    ix=0
#    set +e
#    for k in $(ls versions/ | grep tar.gz | sort -r); do
#      set -e
#      if [[ $ix -ge $RETAIN_LATEST ]]; then
#        show_log d "Cleaning up old zipped version ${CODEPATH}/src/${i}/versions/${k}"
#        rm -r ${CODEPATH}/src/${i}/versions/${k}
#      fi
#      ix=$((ix+1))
#    done
#    set -e
#    cd ${CODEPATH}
#  done
#}

#function do_rollback() {
#  versions_check
#  if [[ ${PREV_VERSION} != "" ]]; then
#    show_log w "Do you want to OVERWRITE the current version ${CURR_VERSION} WITH VERSION ${PREV_VERSION}? (just press y or n)"
#    LOOP=true
#    while [[ $LOOP == true ]] ; do
#      read -r -n 1 -p "[y/n]: " REPLY
#      case $REPLY in
#        [yY])
#          echo
#          build_abort ${CURR_VERSION} ${PREV_VERSION} ${ARCH}
#          LOOP=false
#          ;;
#        [nN])
#          echo
#          LOOP=false
#          ;;
#        *) echo;show_log w "Invalid Input, please answer y or n. Do you want to Overwrite with ${PREV_VERSION}?"
#      esac
#    done
#  else
#    # TODO: possibility to automatically recover a zipped version
#    show_log e "There are no versions available to rollback to. Please do one fo the following:"
#    show_log e " - Correct your code until it can be deployed."
#    show_log e " - Restore a zipped version that may be available"
#  fi
#}

#function old_do_deploy() {
#  versions_check
#  show_log i "##################  \"DEPLOYING\" THE LATEST VERSION  ####################"
#  # Find the latest crossbuilt version
#  FOUND_LATEST_VIABLE=false
#  if [[ $(ls ${CODEPATH}/src/${ROS_PCKGS[0]}/versions/${CURR_VERSION}/ | grep ${CROSSARCH} | wc -l) -ne 1 ]]; then
#    show_log w "Current Version ${CURR_VERSION} has not yet built for ${CROSSARCH}!"
#    show_log w "Please consider running $0 without parameters to create a full build."
#    # TODO: option to use the second latest if available
#  else
#    show_log i "Preparing to deploy Version: ${CURR_VERSION} for arch ${CROSSARCH}"
#
#    cd ${CODEPATH}
#    CWD=$(pwd)
#    for i in ${ROS_PCKGS[@]}; do
#      cd src/${i}
#      for j in log build install; do
#        rm ${j} 2>/dev/null || true 
#        cp -r versions/${CURR_VERSION}/${CROSSARCH}/${j} ${j}
#      done
#      cd ${CWD}
#    done
#  fi
#  LATEST_TAG=$(echo $(git tag -l | tail -n1))
#  PROPOSED=$(echo ${LATEST_TAG} | awk -F '.' '{print $1"."$2"."$3+1}')
#  show_log i "Latest released tag is: ${LATEST_TAG}"
#  read -r -p "Please write down the new release id: (${PROPOSED})" NEWTAG 
#  if [[ ${NEWTAG} == "" ]]; then
#    NEWTAG=${PROPOSED}
#    echo "Using ${NEWTAG}"
#  fi
#  read -r -p "Please also provide a description for the new release: " NEWTAGDESC 
#  git add ${CODEPATH}
#  git commit -m "${NEWTAGDESC}"
#  git push 
#  git tag -a ${NEWTAG} -m "${NEWTAGDESC}"
#  git push --tags
#  ssh -o ConnectTimeout=3 ${NEWUSER}@${SSHIP} -p${SSHPORT} "cd robot ; git pull"
#  show_log i "##################  Robot now also has the latest version installed  ####################"
#}

# AUX FUNCTIONS

function kill_switch() {
  # TODO: this doesnt work well in the container
  for pid in $(ps aux | grep robot.sh | grep -v vim | awk '{print $2}'); do
    kill -9 $pid
  done
  reset
}

function ctrl_c() {
  # once the launch run is stopped with CTRL-C we want to clean up
  echo "** Trapped CTRL-C"

  # Cleanup of GPIO after running
  #echo 0 | sudo tee /sys/class/gpio/gpio${LEDMAIN_PIN}/value >/dev/null
  #echo ${LEDMAIN_PIN} | sudo tee /sys/class/gpio/unexport >/dev/null
  #
  # Cleanup of PIDs
  echo "Killing leftovers..."
  for i in $(ps aux | grep target | grep circuits | awk '{print $2}'); do echo $i;kill $i;done
  kill_switch
  echo "...Killed!"

  exit 2
}

function show_log {
  TSTAMP=$(date "+%Y-%m-%d %H:%M:%S")
  case $1 in
    "debug"|"d")
      #echo "[$TSTAMP][DEBUG] - $2"
      printf '\e[35m%s\n\e[0m' "[$TSTAMP][DEBUG] - $2"
      ;;
    "info"|"i")
      #echo "[$TSTAMP][INFO] - $2"
      printf '\e[32m%s\n\e[0m' "[$TSTAMP][INFO] - $2"
      ;;
    "warn"|"w")
      #echo "[$TSTAMP][WARN] - $2"
      printf '\e[33m%s\n\e[0m' "[$TSTAMP][WARN] - $2"
      ;;
    "error"|"err"|"e")
      #echo "[$TSTAMP][ERROR] - $2"
      printf '\e[31m%s\n\e[0m' "[$TSTAMP][ERROR] - $2"
      ;;
    *)
      #echo "[$TSTAMP][DEBUG] - Wrong Logging mode"
      printf '\e[35m%s\n\e[0m' "[$TSTAMP][DEBUG] - $2"
      exit 2
      ;;
  esac
}

function load_dotenv {
  if [ -f "$CONFIGFILE" ]; then
    export $(cat ${CONFIGFILE} | grep -v '^#' | xargs) >/dev/null
  else 
    show_log e "ERROR! .env file not found!"
    show_log i "REMEMBER you can copy env.template to .env and adapt it!"
    show_log i "Exiting..."
  fi
}

function show_help() {
  show_log i "SYNTAX:"
  #show_log i " $0        - Build, Crosscompile and deploy to robot"
  show_log i " - Main options:"
  show_log i " $0 build     - Build related packages and run test locally"
  #show_log i " $0 test      - Run tests locally"
  #show_log i " $0 buildonly - Build related packages locally, accepts a list of packages"
  #show_log i " $0 cross     - Cross-build for the robot's architecture"
  #show_log i " $0 deploy    - Deploy current crossbuild to robot"
  #show_log i " $0 run       - Run current crossbuild on the robot"
  show_log i " - Other options:"
  #show_log i " $0 rollback  - Deploy previous crossbuild version on robot and make it current"
  #show_log i " $0 clean     - Remove and compress old versions built" 
  #show_log i " $0 reset     - Remove everything previously built " 
  #show_log i " $0 kill      - Kill any leftover processes" 
  #show_log i " $0 version   - Show the current version(s) available" 
  #show_log i " $0 aux       - Run a function that is currently used for development of new stuff" 
  show_log i " $0 help      - Show this help"
}

function aux_function() {
  echo
}

function do_mode() {
  if [[ "$1" == "help" ]]; then
    show_help
  elif [[ "$1" == "buildrun" ]] || [[ "$1" == "buildnrun" ]] || [[ "$1" == "build_n_run" ]]; then
    do_build $@
    #do_test
    do_run
  elif [[ "$1" == "test" ]]; then
    echo do_test
  elif [[ "$1" == "build" ]] || [[ "$1" == "buildonly" ]]; then
    do_build $@
  elif [[ "$1" == "cross" ]] || [[ "$1" == "crossbuild" ]]; then
    echo do_crossbuild
  elif [[ "$1" == "deploy" ]]; then
    echo do_deploy
  elif [[ "$1" == "run" ]]; then
    do_run
  elif [[ "$1" == "rollback" ]]; then
    echo do_rollback
  elif [[ "$1" == "clean" ]]; then
    echo do_clean
  elif [[ "$1" == "reset" ]]; then
    echo versions_reset
  elif [[ "$1" == "kill" ]]; then
    echo kill_switch
  elif [[ "$1" == "version" ]] || [[ "$1" == "versions" ]]; then
    echo versions_show
  elif [[ "$1" == "" ]]; then
    do_build 
    do_run
    #do_test 
    #do_crossbuild
    #do_crossrun
  elif [[ "$1" == "aux" ]]; then
    #This option we use to test functions
    aux_function $@
  else
    show_help
  fi
}

######################

CONFIGFILE=".env"
# We inject ARCH=aarch64 in the cross-compiling container and can reuse build functions
if [[ "${ARCH}" == "" ]]; then
  ARCH=$(uname -m)
fi
#CROSSARCH=aarch64 # This is what Raspberry 3B+ uses
CWDMAIN=$(pwd)
CODEPATH="${CWDMAIN}/circuits"
# TODO: change this when out of circuits folder
#CODEPATH="${CWDMAIN}/circuits"
VERSIONFILE="${CODEPATH}/VERSION"

RUST_PCKGS=($(ls -l ${CODEPATH} | grep "^d" | awk '{print $9}'))
#ROS_PCKGS=($(ls ${CODEPATH}/src))

load_dotenv
do_mode $@
