#!/usr/bin/env bash

ENVFILE="brain/.env"
source ${ENVFILE} 

EXIT=0
TITLE_SHOWN=False

main()
{
  checkArgs $@
  if [[ "${TITLE_SHOWN}" == "False" ]]; then
    echo "${TITLE}"
  fi
  echo -e "${RESULT}"
  exit $EXIT
}

checkArgs()
{
  if [[ "$1" == "get" ]]; then
    getStuff $@
  elif [[ "$1" == "do" ]]; then
    doStuff $@
  else
    echo "ERROR: parameter not recognized: $1"
    help 
    EXIT=1
  fi
}

help()
{
  TITLE="SYNTAX:"
  RESULT="\
$0 get <parameter>\n \
  get \n \
    online - Get if the robot is online \n \
  do \n \
    check - Run a small HW check in your Robot and show logs \n \
    run - Run Brain in your Robot and show logs \n \
    record <filename> - Run Brain in your Robot and save arduino input \n \
      <filename> - (Mandatory) filename to save the arduino input to \n \
    reset - Reset the robot's outputs(motors, LEDs...) \n \
    test - Test Rust Code and run Brain locally on test mode \n \
      <filename> - (Mandatory) filename to load our setup from \n \
      <-v> - (Optional) Shows cargo test output and logs from the Robot on DEBUG mode \n \
      <-vv> - (Optional) Shows cargo test output with --nocapture and logs from the Robot on TRACE mode \n \
    compile - Compile code for the Raspberry processor \n \
    gitpush - Add latest changes and push to branch on .env \n \
"
}

getStuff()
{
  if [[ "$2" == "" ]]; then
    echo "ERROR: missing parameter after $1"
    help 
    EXIT=1
  elif [[ "$2" == "online" ]]; then
    getIsOnline
  else
    echo "ERROR: parameter not recognized: $2"
    help 
    EXIT=1
  fi
}

getIsOnline()
{
  TITLE="ITEM    IS_ONLINE"
  online="False"
  retries=0
  while [[ $retries < 4 ]] && [ "$online" != "True" ]; do
    retries=$((retries+1))
    ping ${HOST} -c 1 > /dev/null
    if [[ $? -eq 0 ]]; then
      online="True"
    fi
  done
  if [[ "$online" == "True" ]]; then
    RESULT="Robot   True"
    EXIT=0
  else
    RESULT="Robot   False"
    EXIT=1
  fi
}

doStuff()
{
  if [[ "$2" == "" ]]; then
    echo "ERROR: missing parameter after $1"
    help 
    EXIT=1
  elif [[ "$2" == "check" ]]; then
    doCheck $@
  elif [[ "$2" == "reset" ]]; then
    doReset $@
  elif [[ "$2" == "compile" ]]; then
    doCompile $@
  elif [[ "$2" == "gitpush" ]]; then
    doGitPush $@
  elif [[ "$2" == "run" ]]; then
    doRun $@
  elif [[ "$2" == "test" ]]; then
    if [[ "$3" == "" ]]; then
      echo "ERROR: filename missing"
      help 
      EXIT=1
    else
      doTest $@
    fi
  elif [[ "$2" == "record" ]]; then
    if [[ "$3" == "" ]]; then
      echo "ERROR: filename missing"
      help 
      EXIT=1
    else
      doRecord $@
    fi
  else
    echo "ERROR: parameter not recognized: $2"
    help 
    EXIT=1
  fi
}

doReset()
{
  getIsOnline
  TITLE="ITEM    RESET"
  if [[ "${RESULT}" != "Robot   True" ]]; then
      RESULT="Robot   Error: NOT Online"
  else
    ${SSH_COMM} "kill \$(ps aux | grep brain | grep setup | awk '{print \$2}')" > /dev/null 2>&1
    ${SSH_COMM} "cd robot/brain; \
      RUST_LOG=info target/arm-unknown-linux-gnueabihf/debug/brain reset setup_reset.yaml
      " > /dev/null 2>&1
      EXIT=$?
    if [[ $EXIT -eq 0 ]]; then
      ${SSH_COMM} "kill \$(ps aux | grep brain | grep cfg | awk '{print \$2}')"
      RESULT="Robot   True"
    else
      RESULT="Robot   Error: "$EXIT
    fi
  fi
}

doCompile()
{
  TITLE="ITEM    COMPILE"
  CWD=$(pwd)
  cd brain
  if [[ "$3" == "-v" ]]; then
    cross build --target=arm-unknown-linux-gnueabihf
    EXIT=$?
  else
    cross build --target=arm-unknown-linux-gnueabihf > /dev/null 2>&1
    EXIT=$?
  fi
  cd ${CWD}
  if [[ $EXIT -eq 0 ]]; then
    RESULT="Robot   True"
  else
    RESULT="Robot   Error: "$EXIT
  fi
}

doGitPush()
{
  TITLE="ITEM    PUSHED_TO_GIT"
  git checkout ${DEV_BRANCH} > /dev/null 2>&1
  git add .  > /dev/null 2>&1
  git commit -m "updating and uploading" > /dev/null 2>&1
  git push origin ${DEV_BRANCH} > /dev/null 2>&1
  EXIT=$?
  if [[ $EXIT -eq 0 ]]; then
    RESULT="Robot   True"
  else
    RESULT="Robot   Error: "$EXIT
  fi
}

doCheck()
{
  RESULT="Robot   Check ..."
  echo -e "${RESULT}"
  ${SSH_COMM} "cd robot/brain; git pull; git checkout ${DEV_BRANCH} && git stash; git stash drop; git pull && \
    RUST_LOG=info target/arm-unknown-linux-gnueabihf/debug/brain live setup.yaml
    "
  EXIT=$?
  if [[ $EXIT -eq 0 ]]; then
    RESULT="Robot   True"
  else
    RESULT="Robot   Error: "$EXIT
  fi
}

doRun()
{
  doCompile $@
  TITLE="ITEM    RUN"
  echo "${TITLE}"
  TITLE_SHOWN="True"
  if [[ "${RESULT}" != "Robot   True" ]]; then
    RESULT="Robot   Error: Compilation failed"
  else
    RESULT="Robot   OK: Compilation"
    echo -e "${RESULT}"
    doGitPush
    if [[ "${RESULT}" != "Robot   True" ]]; then
      RESULT="Robot   Error: Updating Git branch failed"
    else
      RESULT="Robot   OK: Updating git branch"
      echo -e "${RESULT}"
      doReset
      if [[ "${RESULT}" != "Robot   True" ]]; then
        RESULT="Robot   Error: Reset failed"
      else
        RESULT="Robot   OK: Resetting"
        echo -e "${RESULT}"
        RESULT="Robot   Run ..."
        echo -e "${RESULT}"
        ${SSH_COMM} "cd robot/brain; git pull; git checkout ${DEV_BRANCH} && git stash; git stash drop; git pull && \
          RUST_LOG=info target/arm-unknown-linux-gnueabihf/debug/brain live setup.yaml
          "
        EXIT=$?
        if [[ $EXIT -eq 0 ]]; then
          RESULT="Robot   True"
        else
          RESULT="Robot   Error: "$EXIT
        fi
      fi
    fi
  fi
}

doTest()
{
  TITLE="ITEM    TEST_PASSED"
  FILE=$3
  PATHPREFIX="brain/" # we are going to cd into brain anyway
  FILE=${FILE#"$PATHPREFIX"}
  PATHPREFIX="./brain/" # we are going to cd into brain anyway
  FILE=${FILE#"$PATHPREFIX"}
  echo "${TITLE}"
  TITLE_SHOWN="True"
  CWD=$(pwd)
  cd brain
  if [[ "$4" == "-v" ]]; then
    cargo test -- 
    EXIT=$?
  elif [[ "$4" == "-vv" ]]; then
    cargo test -- --nocapture
    EXIT=$?
  else
    cargo test -- > /dev/null 2>&1
    EXIT=$?
  fi
  cd ${CWD}
  if [[ $EXIT -eq 0 ]]; then
    RESULT="Robot   True"
    echo "${RESULT}"
    echo 
    TITLE="ITEM    OFFLINE_TEST"
    echo "${TITLE}"
    RESULT="Robot   Running..."
    echo "${RESULT}"
    CWD=$(pwd)
    cd brain
    if [[ "$4" == "-v" ]]; then
      RUST_LOG=debug cargo run test "${FILE}"
      EXIT=$?
    elif [[ "$4" == "-vv" ]]; then
      RUST_LOG=trace cargo run test "${FILE}"
      EXIT=$?
    else
      RUST_LOG=info cargo run test "${FILE}"
      EXIT=$?
    fi
    cd ${CWD}
    if [[ $EXIT -eq 0 ]]; then
      RESULT="Robot   True"
    else
      RESULT="Robot   Error: "$EXIT
    fi
  else
    RESULT="Robot   Error: "$EXIT
  fi
}

doRecord()
{
  FILE=$3
  if [ -f ${FILE} ]; then
    echo "You are about to overwrite ${FILE}!"
    read -r -p "Are you sure? [y/N] " response
    case "$response" in
      [nN][oO]|[nN]) 
        echo "Stopping then."
        exit 1
        ;;
      *)
        echo "Overwriting then."
        TITLE="ITEM    RUN_AND_RECORD"
        echo "${TITLE}"
        TITLE_SHOWN="True"
        doCompile $@
        if [[ "${RESULT}" != "Robot   True" ]]; then
          RESULT="Robot   Error: Compilation failed"
        else
          RESULT="Robot   OK: Compilation"
          echo -e "${RESULT}"
          doGitPush
          if [[ "${RESULT}" != "Robot   True" ]]; then
            RESULT="Robot   Error: Updating Git branch failed"
          else
            RESULT="Robot   OK: Updating git branch"
            echo -e "${RESULT}"
            doReset
            if [[ "${RESULT}" != "Robot   True" ]]; then
              RESULT="Robot   Error: Reset failed"
            else
              RESULT="Robot   OK: Resetting"
              echo -e "${RESULT}"
              RESULT="Robot   Running... (please wait, it might take up to a minute)"
              echo -e "${RESULT}"
              ${SSH_COMM} "cd robot/brain; git pull; git checkout ${DEV_BRANCH} && git stash; git stash drop; git pull && \
                RUST_LOG=info target/arm-unknown-linux-gnueabihf/debug/brain record setup.yaml
                " > /dev/null 2>&1
              scp ${SSH_CONN}:robot/brain/records/last_run.yaml ${FILE} > /dev/null 2>&1
              EXIT=$?

              if [[ $EXIT -eq 0 ]]; then
                RESULT="Robot   OK: Saved to ${FILE}"
                doReset
                if [[ "${RESULT}" != "Robot   True" ]]; then
                  RESULT="Robot   Error: Reset failed"
                else
                  RESULT="Robot   OK: Resetting"
                fi
              else
                RESULT="Robot   Error: "$EXIT
              fi
            fi
          fi
        fi
        ;;
    esac
  else
    TITLE="ITEM    RUN_AND_RECORD"
    echo "${TITLE}"
    TITLE_SHOWN="True"
    doCompile $@
    if [[ "${RESULT}" != "Robot   True" ]]; then
      RESULT="Robot   Error: Compilation failed"
    else
      RESULT="Robot   OK: Compilation"
      echo -e "${RESULT}"
      doGitPush
      if [[ "${RESULT}" != "Robot   True" ]]; then
        RESULT="Robot   Error: Updating Git branch failed"
      else
        RESULT="Robot   OK: Updating git branch"
        echo -e "${RESULT}"
        doReset
        if [[ "${RESULT}" != "Robot   True" ]]; then
          RESULT="Robot   Error: Reset failed"
        else
          RESULT="Robot   OK: Resetting"
          echo -e "${RESULT}"
          RESULT="Robot   Running... (please wait, it might take up to a minute)"
          echo -e "${RESULT}"
          ${SSH_COMM} "cd robot/brain; git pull; git checkout ${DEV_BRANCH} && git stash; git stash drop; git pull && \
            RUST_LOG=info target/arm-unknown-linux-gnueabihf/debug/brain record setup.yaml
            " > /dev/null 2>&1
          scp ${SSH_CONN}:robot/brain/records/last_run.yaml ${FILE} > /dev/null 2>&1
          EXIT=$?

          if [[ $EXIT -eq 0 ]]; then
            RESULT="Robot   OK: Saved to ${FILE}"
            doReset
            if [[ "${RESULT}" != "Robot   True" ]]; then
              RESULT="Robot   Error: Reset failed"
            else
              RESULT="Robot   OK: Resetting"
            fi
          else
            RESULT="Robot   Error: "$EXIT
          fi
        fi
      fi
    fi

  fi
}

main $@
