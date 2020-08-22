#!/bin/bash

source .env

git checkout ${DEV_BRANCH}
git add ${ARDUINO_FILES}
git add src/
git commit -m "ready to test"
git push origin ${DEV_BRANCH}


${SSH_COMM} "cd robot/brain; pwd; git checkout ${DEV_BRANCH} && git pull && ${CARGO} test -- --ignored"
