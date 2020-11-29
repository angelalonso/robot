#!/bin/bash
die () {
    echo >&2 "$@"
    exit 1
}

[ "$#" -eq 1 ] || die "ERROR: please provide the name of the local file to save the recorded run to, $# provided"

FILE=$1
if [ -f ${FILE} ]; then
    echo "You are about to overwrite ${FILE}!"
    read -r -p "Are you sure? [y/N] " response
    case "$response" in
        [nN][oO]|[nN]) 
            echo "Stopping then."
            exit 1
            ;;
        *)
            echo "Continuing!"
            ;;
    esac

fi

source .env
./build_for_raspberry.sh
./do_reset.sh

git checkout ${DEV_BRANCH}
git add ${ARDUINO_FILES}
#git add -f target/arm-unknown-linux-musleabi/debug/brain
git add -f target/arm-unknown-linux-gnueabihf/debug/brain
git add setup.yaml
git add move_cfg.yaml
git add src/
git add rulesets/
git add Cargo.toml
git commit -m "changing and installing"
git push origin ${DEV_BRANCH}

online=false
while [ $online = false ]
do
  echo "trying to reach the robot..."
  ping ${HOST} -c 1
  if [[ $? -eq 0 ]]
    then
      online=true
  fi
done

ssh=false
while [ $online = false ]
do
  echo "trying to reach the robot's SSH..."
  ${SSH_COMM} "echo $PWD"
  if [[ $? -eq 0 ]]
    then
      online=true
  fi
done
${SSH_COMM} "cd robot/brain; git pull; git checkout ${DEV_BRANCH} && git pull && \
  RUST_LOG=info target/arm-unknown-linux-gnueabihf/debug/brain record setup.yaml
  "
scp ${SSH_CONN}:robot/brain/records/last_run.yaml ${FILE} 
# TODO: ask for a destination file name on run
