#!/usr/bin/env bash
#
# TODO: we should only run robot.sh and it should follow the default mode:
#   - Build -> Test -> Crossbuild -> Deploy -> Run
#   The point is that it should progress based on checks:
#   - Am I running on the Raspberry machine?
#   - Is the Robot available through SSH?
#   - Did all steps finish properly?
#   TODO: Next up: Test build error
#   TODO: Cross build
#   TODO: Deploy
#   TODO: Run at the robot

set -eo pipefail
shopt -s extglob # required for proper string substitution

# ACTION FUNCTIONS

function do_build() {
  show_log i "################## BUILD ####################"
  trap ctrl_c INT

  # Build only if anything changed (comparing to git), 
  # TODO: we need to define this "git control" better (what if it was already commited?)
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
    BUILDTSTAMP=$(date "+%Y%m%d_%H%M%S") # to be used to identify build
    
    cd ${CODEPATH}
    source /opt/ros/rolling/local_setup.sh
    rm -rf log/* build/* install/*

    # TODO: maybe create a second one list if folders for rust modules in the future
    CWD=$(pwd)
    for i in ${ROS_PCKGS[@]}; do
      show_log i "Building ${i}"
      cd src/${i}
      rm log build install 2>/dev/null || true 
      colcon build && \
      . ./install/setup.bash
      # Make sure paths exist
      # TODO: only do this if nothing failed to build, otherwise rollback
      # TODO: maybe the whole rollback thing should go on another function
      mkdir -p ${ARCH}/install
      mkdir -p ${ARCH}/log
      mkdir -p ${ARCH}/build
      mv install ${ARCH}/install/${BUILDTSTAMP}
      mv log ${ARCH}/log/${BUILDTSTAMP}
      mv build ${ARCH}/build/${BUILDTSTAMP}
      ln -s ${ARCH}/install/${BUILDTSTAMP} install
      ln -s ${ARCH}/log/${BUILDTSTAMP} log
      ln -s ${ARCH}/build/${BUILDTSTAMP} build
      cd $CWD
    done
    # TODO: check the build worked before continuing here
    show_log i "######## Built Version: ${BUILDTSTAMP} ########"

  else
    show_log w "Nothing was built"
  fi
  cd $CWDMAIN
  # TODO: only do this if nothing failed to build
  # do_clean
}

function do_test() {
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
    cd src/${i}/${ARCH}
    for j in log install build; do
      # STEP 1: Leave only the latest 3 directories
      RETAIN_LATEST=3
      ix=0
      set +e
      for k in $(ls -d ${CODEPATH}/src/${i}/${ARCH}/${j}/*/ 2>/dev/null | sort -r ); do
        set -e
        #show_log d "checking ${k}"
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
      for k in $(ls -d ${CODEPATH}/src/${i}/${ARCH}/${j}/*/ 2>/dev/null | sort -r ); do
        set -e
        #show_log d "checking ${CODEPATH}/src/${i}/${ARCH}/${j}/${k}"
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
      for k in $(ls  ${j}/ | grep tar.gz | sort -r); do
        set -e
        if [[ $ix -ge $RETAIN_LATEST ]]; then
          show_log d "Cleaning up old zipped version ${CODEPATH}/src/${i}/${ARCH}/${j}/${k}"
          rm -r ${CODEPATH}/src/${i}/${ARCH}/${j}/${k}
        fi
        ix=$((ix+1))
      done
      set -e
    done
    cd ${CODEPATH}
  done
}

function do_rollback() {
  show_log i "########  ROLLING BACK TO PREVIOUS VERSION  ##########"
  trap ctrl_c INT
  # Getting current version # TODO: move to a function, use it script-wide
  CURR_LINK=$(readlink ${CODEPATH}/src/${ROS_PCKGS[0]}/build)
  CURR_VERSION="${CURR_LINK#@(${ARCH}/build/)}"

  # Getting available versions
  PREV_LINKS=$(ls -d ${CODEPATH}/src/${ROS_PCKGS[0]}/${ARCH}/build/*/ 2>/dev/null | sort -r )
  DEPLOYORNOT=false
  NOTHING_TO_DEPLOY=true
  for i in ${PREV_LINKS}; do
    FMT_i=${i::-1} # Removing the / at the end
    PREV_VERSION="${FMT_i#@(${CODEPATH}/src/${ROS_PCKGS[0]}/${ARCH}/build/)}"
    if [[ "${DEPLOYORNOT}" == false ]]; then
      if [[ ${PREV_VERSION} != ${CURR_VERSION} ]]; then
        NOTHING_TO_DEPLOY=false
        show_log w "Do you want to OVERWRITE the current version ${CURR_VERSION} WITH VERSION ${PREV_VERSION}? (just press y or n)"
        LOOP=true
        while [[ $LOOP == true ]] ; do
          read -r -n 1 -p "[y/n]: " REPLY
          case $REPLY in
            [yY])
              echo
              DEPLOYORNOT=true
              cd ${CODEPATH}
              CWD=$(pwd)
              echo ${ROS_PCKGS[@]}
              for i in ${ROS_PCKGS[@]}; do
                cd src/${i}
                show_log i "Cleaning up ${i}"
                # remove current links
                rm log build install 2>/dev/null || true 
                # link to the previous version
                ln -s ${ARCH}/install/${PREV_VERSION} install
                ln -s ${ARCH}/log/${PREV_VERSION} log
                ln -s ${ARCH}/build/${PREV_VERSION} build
                # TODO: maybe test before removing
                rm -r ${ARCH}/install/${CURR_VERSION}
                rm -r ${ARCH}/log/${CURR_VERSION}
                rm -r ${ARCH}/build/${CURR_VERSION}
                cd $CWD
              done
              LOOP=false
              ;;
            [nN])
              echo
              DEPLOYORNOT=false
              LOOP=false
              ;;
            *) echo;show_log w "Invalid Input, please answer y or n. Do you want to Overwrite with ${PREV_VERSION}?"
          esac
        done
      fi
    fi
  done
  if [[ "${NOTHING_TO_DEPLOY}" == true ]]; then
    show_log w "No previous version was found to rollback to! We are stuck with ${CURR_VERSION}"
  fi
}

function crossbuild() {
  show_log i "##################  CROSS BUILD  ####################"
  trap ctrl_c INT

  rm -rf ../cross_circuits 
  cp -R ../circuits ../cross_circuits

  docker build \
    --build-arg NEWUSER=$USER \
    --platform linux/arm64/v8 \
    -t aarch64-cross .
  docker run \
    --rm \
    --user $USER \
    --platform linux/arm64/v8 \
    -v $PWD/../cross_circuits:/home/$USER/ros2_ws \
    -v "$PWD:/work" \
    -it aarch64-cross \
    /bin/bash -c cd ros2_ws && ./run.sh build
}

function deploy() {
# TODO: 
#  not found: "/home/aaf/ros2_ws/src/action_interfaces/install/local_setup.bash"
#  not found: "/home/aaf/ros2_ws/src/action_interfaces/install/local_setup.bash"
#  not found: "/home/aaf/ros2_ws/src/action_servers/install/local_setup.bash"
# TODO: use rsync ?
  scp -P ${SSHPORT} -r -C ../cross_circuits ${NEWUSER}@${SSHIP}:/home/${NEWUSER}/robot/cross_circuits
}

function just_run() {
  show_log i "##################  RUN  ####################"
  trap ctrl_c INT

  source /opt/ros/rolling/local_setup.sh

  CWD=$(pwd)
  cd src/action_interfaces
  . ./install/setup.bash && \
  cd $CWD
  cd src/action_servers
  . ./install/setup.bash && \
  cd $CWD
  cd src/circuit_nodes
  . ./install/setup.bash && \
    ros2 launch circuit_nodes circuits.launch.py
  cd $CWD
}

function build_n_test() {
  build
  just_test
}

function do_all() {
  # TODO: check previous step worked before moving to next
  do_build
  do_test
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
    do_build_n_test
  elif [[ "$1" == "cross" ]] || [[ "$1" == "crossbuild" ]]; then
    do_crossbuild
  elif [[ "$1" == "deploy" ]]; then
    do_deploy
  elif [[ "$1" == "run" ]]; then
    do_run
  elif [[ "$1" == "clean" ]]; then
    # TODO: include this on all other functions that need it, then remove this option
    do_clean
  elif [[ "$1" == "rollback" ]]; then
    do_rollback
  else
    do_all
  fi
}

ARCH=$(uname -m)
CWDMAIN=$(pwd)
CODEPATH="${CWDMAIN}/circuits"
ROS_PCKGS=($(ls ${CODEPATH}/src))

check_dotenv
do_mode $1
