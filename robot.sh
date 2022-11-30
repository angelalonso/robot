#!/usr/bin/env bash
#
# TODO: we should only run robot.sh and it should follow the default mode:
#   - Build -> Test -> Crossbuild -> Deploy -> Run
#   The point is that it should progress based on checks:
#   - Am I running on the Raspberry machine?
#   - Is the Robot available through SSH?
#   - Did all steps finish properly?
#   TODO: Next up: Make sure build_prepare and related are used on build, deploy, rollback...
#   TODO: Cross build
#   TODO: Deploy
#   TODO: Run at the robot

set -eo pipefail
shopt -s extglob # required for proper string substitution


# VERSION MANAGEMENT FUNCTIONS

function versions_reset() {
    show_log w "This mode erases all past builds! are you sure?? (just press y or n)"
    LOOP=true
    while [[ $LOOP == true ]] ; do
      read -r -n 1 -p "[y/n]: " REPLY
      case $REPLY in
        [yY])
          echo
          cd ${CODEPATH}
          echo > ${VERSIONFILE}
          for i in ${ROS_PCKGS[@]}; do
            show_log i "Cleaning up ${i}"
            for j in log install build; do
              rm src/${i}/${j} 2>/dev/null || true 
              rm -r src/${i}/versions/* || true
            done
          done
          LOOP=false
          ;;
        [nN])
          echo
          LOOP=false
          ;;
        *) echo;show_log w "Invalid Input, please answer y or n. Do you want to Build?"
      esac
    done
}

function versions_check() {
  # Future version
  NEXT_VERSION=$(date "+%Y%m%d_%H%M%S") # to be used to identify build

  # Current version
  SAVEDVERSION=$(cat ${VERSIONFILE} 2>/dev/null || true)
  CURR_LINK=$(readlink ${CODEPATH}/src/${ROS_PCKGS[0]}/build || true)
  CURR_VERSION="$( echo "$CURR_LINK" | sed -e 's#^versions/##; s#/'${ARCH}'/build##; s#/'${CROSSARCH}'/build##' )"
  if [[ "${SAVEDVERSION}" != "${CURR_VERSION}" ]]; then
    show_log w "Saved Version and Installed Version differ!"
    show_log w " ${SAVEDVERSION} vs. ${CURR_VERSION}"
    show_log w "Do you want to Correct the Saved Version? (just press y or n)"
    LOOP=true
    while [[ $LOOP == true ]] ; do
      read -r -n 1 -p "[y/n]: " REPLY
      case $REPLY in
        [yY])
          echo
          echo ${CURR_VERSION} > ${VERSIONFILE}
          LOOP=false
          ;;
        [nN])
          echo
          LOOP=false
          ;;
        *) echo;show_log w "Invalid Input, please answer y or n. Do you want to Correct it?"
      esac
    done
  fi

  # Previous version
  PREV_LINKS=$(ls -d ${CODEPATH}/src/${ROS_PCKGS[0]}/versions/*/ 2>/dev/null || true | sort -r )
  for i in ${PREV_LINKS}; do
    FMT_i=${i::-1} # Removing the / at the end
    PREV_VERSION_UNTESTED="${FMT_i#@(${CODEPATH}/src/${ROS_PCKGS[0]}/versions/)}"
    if [[ ${PREV_VERSION_UNTESTED} != ${CURR_VERSION} ]]; then
      PREV_VERSION=${PREV_VERSION_UNTESTED}
    fi
  done
}

function versions_show() {
  versions_check
  show_log i "Current Version = ${CURR_VERSION}"
  show_log i "Rollback Version = ${PREV_VERSION}"
  ALL_VERSIONS=$(ls ${CODEPATH}/src/${ROS_PCKGS[0]}/versions/ 2>/dev/null || true | sort )
  for i in ${ALL_VERSIONS}; do
    show_log d "${i}"
  done
}


# BUILDS MANAGEMENT FUNCTIONS

function build_prepare() {
  # $1 - Next Version
  # $2 - Architecture
  show_log i "Preparing to deploy Version: ${1} for arch ${2}"

  # TODO: maybe create a second one list if folders for rust modules in the future
  cd ${CODEPATH}
  CWD=$(pwd)
  for i in ${ROS_PCKGS[@]}; do
    cd src/${i}
    for j in log build install; do
      rm ${j} 2>/dev/null || true 
      mkdir -p versions/${1}/${2}/${j}
    done
    cd ${CWD}
  done
}

function build_abort() {
  # $1 - Next Version
  # $2 - Current Version
  # $3 - Architecture
  show_log w "Removing Version: ${1} completely (on ${3})."
  show_log w "Re-enabling Version: ${2}"

  # TODO: maybe create a second one list if folders for rust modules in the future
  cd ${CODEPATH}
  CWD=$(pwd)
  for i in ${ROS_PCKGS[@]}; do
    cd src/${i}
    for j in log build install; do
      rm -r ${j} 
      if [[ "${2}" != "" ]]; then
        ln -s versions/${2}/${3}/${j} ${j}
        rm -r versions/${1} 2>/dev/null || true # TODO: probably this should go out of a loop, do we need the loop at all?
      else
        show_log d "No other version was currently deployed before this build."
      fi
    done
    cd ${CWD}
  done
  echo ${2} > ${VERSIONFILE}
}

function build_confirm() {
  # $1 - Next Version
  # $2 - Architecture
  show_log i "Deployed Version ${1} successfully! (arch ${2})"

  cd ${CODEPATH}
  CWD=$(pwd)
  for i in ${ROS_PCKGS[@]}; do
    show_log i "Deploying ${i}"
    cd src/${i}
    for j in log build install; do
      mv ${j} versions/${1}/${2}/
      ln -s versions/${1}/${2}/${j} ${j}
    done
    cd ${CWD}
  done
  echo ${1} > ${VERSIONFILE}
  do_clean
}


# ACTION FUNCTIONS

function do_build() {
  versions_check
  show_log i "################## BUILD ####################"
  trap ctrl_c INT

  # Build only if anything changed (comparing to git), 
  # TODO: we need to define this "git control" better (what if it was already commited?)
  # TODO: One error is: as long as you dont commit, you can build without being asked, even if you didnt change since latest commit
  BUILDORNOT=false
  if [[ $(git status -s ${CODEPATH} | wc -l) -gt 0 ]]; then
    BUILDORNOT=true
  else
    show_log w "There seems to be no changes on ${CODEPATH}"
    show_log w "Do you still want to Build? (just press y or n)"
    LOOP=true
    while [[ $LOOP == true ]] ; do
      read -r -n 1 -p "[y/n]: " REPLY
      case $REPLY in
        [yY])
          echo
          BUILDORNOT=true
          LOOP=false
          ;;
        [nN])
          echo
          BUILDORNOT=false
          LOOP=false
          ;;
        *) echo;show_log w "Invalid Input, please answer y or n. Do you want to Build?"
      esac
    done
  fi

  if [[ "${BUILDORNOT}" == true ]]; then
    cd ${CODEPATH}
    source /opt/ros/rolling/local_setup.sh
    build_prepare ${NEXT_VERSION} ${ARCH}

    # TODO: maybe create a second one list if folders for rust modules in the future
    CWD=$(pwd)
    NO_BUILD_ERRORS=true
    for i in ${ROS_PCKGS[@]}; do
      show_log i "Building ${i}"
      cd src/${i}
 
      set +e
      colcon build
      if [[ $? -eq 0 ]]; then
        . ./install/setup.bash
      else
        NO_BUILD_ERRORS=false
        break
      fi
      set -e

      cd $CWD
    done
    # check the build worked before deploying the build
    if [[ "${NO_BUILD_ERRORS}" == true ]]; then
      show_log i "######## Built Version: ${NEXT_VERSION} ########"
      build_confirm ${NEXT_VERSION} ${ARCH}
    else
      show_log e "######## There was an error building ${NEXT_VERSION} ########"
      show_log e "######## Back to ${CURR_VERSION} ########"
      build_abort ${NEXT_VERSION} ${CURR_VERSION} ${ARCH}
    fi
  else
    show_log w "Nothing was built"
  fi
  cd $CWDMAIN
}

function do_test() {
  #TODO: make sure arch is correct
  show_log i "##################  RUN A LOCAL TEST  ####################"
  trap ctrl_c INT

  source /opt/ros/rolling/local_setup.sh

  cd ${CODEPATH}
  for i in ${ROS_PCKGS[@]}; do
    show_log i "Loading ${i}"
    cd src/${i}
    . ./install/setup.bash && \
    cd ${CODEPATH}
  done
  ros2 launch circuit_nodes circuits.launch.py
}

function do_clean() {
  # TODO: tell the user if nothing was cleaned up too
  # Keep the two latest folders, compress the third latest, remove the rest
  show_log i "##################  CLEANUP OLD BUILDS  ####################"
  trap ctrl_c INT

  cd ${CODEPATH}
  for i in ${ROS_PCKGS[@]}; do
    show_log i "Cleaning up ${i}"
    cd src/${i}
    # STEP 1: Leave only the latest 3 directories
    RETAIN_LATEST=3
    ix=0
    set +e
    for k in $(ls -d ${CODEPATH}/src/${i}/versions/*/ 2>/dev/null | sort -r ); do
      set -e
      if [[ $ix -ge $RETAIN_LATEST ]]; then
        show_log d "Removing old version ${k}"
        rm -r ${k}
      fi
      ix=$((ix+1))
    done
    set -e
    # STEP 2: Compress the third latest directory
    RETAIN_LATEST=2
    ix=0
    set +e
    for k in $(ls -d ${CODEPATH}/src/${i}/versions/*/ 2>/dev/null | sort -r ); do
      set -e
      if [[ $ix -ge $RETAIN_LATEST ]]; then
        show_log d "Compressing ${k::-1}"
        tar -zcf ${k::-1}.tar.gz ${k::-1} 2>/dev/null
        show_log d "Cleaning up original folder ${k::-1}"
        rm -r ${k::-1}
      fi
      ix=$((ix+1))
    done
    set -e
    # STEP 3: Make sure we only keep that compressed file
    RETAIN_LATEST=1
    ix=0
    set +e
    for k in $(ls versions/ | grep tar.gz | sort -r); do
      set -e
      if [[ $ix -ge $RETAIN_LATEST ]]; then
        show_log d "Cleaning up old zipped version ${CODEPATH}/src/${i}/versions/${k}"
        rm -r ${CODEPATH}/src/${i}/versions/${k}
      fi
      ix=$((ix+1))
    done
    set -e
    cd ${CODEPATH}
  done
}

function do_rollback() {
  # TODO: possibility to unzip the last version saved automatically
  versions_check
  if [[ ${PREV_VERSION} != "" ]]; then
    show_log w "Do you want to OVERWRITE the current version ${CURR_VERSION} WITH VERSION ${PREV_VERSION}? (just press y or n)"
    LOOP=true
    while [[ $LOOP == true ]] ; do
      read -r -n 1 -p "[y/n]: " REPLY
      case $REPLY in
        [yY])
          echo
          build_abort ${CURR_VERSION} ${PREV_VERSION} ${ARCH}
          LOOP=false
          ;;
        [nN])
          echo
          LOOP=false
          ;;
        *) echo;show_log w "Invalid Input, please answer y or n. Do you want to Overwrite with ${PREV_VERSION}?"
      esac
    done
  else
    # TODO: possibility to automatically recover a zipped version
    show_log e "There are no versions available to rollback to. Please do one fo the following:"
    show_log e " - Correct your code until it can be deployed."
    show_log e " - Restore a zipped version that may be available"
  fi
}

function do_crossbuild() {
  show_log i "##################  CROSS BUILD  ####################"
  trap ctrl_c INT

  # uncomment this to get the image!
  #docker build \
  #  --build-arg NEWUSER=$USER \
  #  --platform linux/arm64/v8 \
  #  -t aarch64-cross .

  cd ${CWDMAIN}
  docker run \
    --rm \
    --user $USER \
    --platform linux/arm64/v8 \
    -v $PWD:/home/$USER/ros2_ws \
    -e "ARCH=aarch64" \
    -it aarch64-cross \
    /bin/bash -c "cd ros2_ws && ./robot.sh build"

  # TODO: add some tests

}

function deploy() {
# TODO: use rsync ?
  scp -P ${SSHPORT} -r -C ../cross_circuits ${NEWUSER}@${SSHIP}:/home/${NEWUSER}/robot/cross_circuits
}

# AUX FUNCTIONS

function ctrl_c() {
  # once the launch run is stopped with CTRL-C we want to clean up
  echo "** Trapped CTRL-C"

#  # Set back to real libraries to have them ready to be pushed to the repo
#  sed -i 's/^#import RPi.GPIO as GPIO/import RPi.GPIO as GPIO/g' scripts/*.py
#  sed -i 's/^from fake_rpi import fake_rpi as GPIO/#from fake_rpi import fake_rpi as GPIO/g' scripts/*.py
#
#  # Use production python nodes
#  cp scripts/node_arduino.py.prod scripts/node_arduino.py
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

function check_dotenv {
  if [ ! -f ".env" ]; then
    show_log e "ERROR! .env file not found!"
    show_log i "REMEMBER you can copy env.template to .env and adapt it!"
    show_log i "Exiting..."
    exit 2
  fi
}

function show_help() {
  show_log i "SYNTAX:"
  show_log i "  build    - Build related packages locally"
  show_log i "  test     - Run tests locally"
  show_log i "  cross    - Cross-build for the robot's architecture"
  show_log i "  deploy   - Deploy current crossbuild to robot"
  show_log i "  run      - Run current crossbuild on the robot"
  show_log i "  rollback - Deploy previous crossbuild version on robot and make it current"
  show_log i "  help     - Show this help"
}

function do_mode() {
  if [[ "$1" == "help" ]]; then
    show_help
  elif [[ "$1" == "build" ]]; then
    do_build
  elif [[ "$1" == "test" ]]; then
    do_test
  elif [[ "$1" == "buildtest" ]] || [[ "$1" == "buildntest" ]] || [[ "$1" == "build_n_test" ]]; then
    do_build
    do_test
  elif [[ "$1" == "cross" ]] || [[ "$1" == "crossbuild" ]]; then
    do_crossbuild
  elif [[ "$1" == "deploy" ]]; then
    # TODO: remove links and copy over the latest crosscompiled
    do_deploy
  elif [[ "$1" == "run" ]]; then
    # TODO: Check that it doesn't fail because of arch
    do_run
  elif [[ "$1" == "clean" ]]; then
    do_clean
  elif [[ "$1" == "reset" ]]; then
    versions_reset
  elif [[ "$1" == "rollback" ]]; then
    do_rollback
  elif [[ "$1" == "version" ]] || [[ "$1" == "versions" ]]; then
    versions_show
  elif [[ "$1" == "" ]]; then
    # TODO: Add more tests, check previous step worked before moving to next
    do_build
    do_test
    do_crossbuild
  elif [[ "$1" == "aux" ]]; then
    #This option we use to test functions
    show_help
  else
    show_help
  fi
}

# We inject ARCH=aarch64 in the cross-compiling container and can reuse build functions
if [[ "${ARCH}" == "" ]]; then
  ARCH=$(uname -m)
fi
CROSSARCH=aarch64 # This is what Raspberry 3B+ uses
CWDMAIN=$(pwd)
CODEPATH="${CWDMAIN}/circuits"
VERSIONFILE="${CODEPATH}/VERSION"

ROS_PCKGS=($(ls ${CODEPATH}/src))

check_dotenv
do_mode $1
