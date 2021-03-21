use std::process::Command;

static BRAIN_DIR: &str = "../brain";

pub fn run() {
    compile();
//  doCompile $@
//  TITLE="ITEM    RUN"
//  echo "${TITLE}"
//  TITLE_SHOWN="True"
//  if [[ "${RESULT}" != "Robot   True" ]]; then
//    RESULT="Robot   Error: Compilation failed"
//  else
//    RESULT="Robot   OK: Compilation"
//    echo -e "${RESULT}"
//    doGitPush
//    if [[ "${RESULT}" != "Robot   True" ]]; then
//      RESULT="Robot   Error: Updating Git branch failed"
//    else
//      RESULT="Robot   OK: Updating git branch"
//      echo -e "${RESULT}"
//      doReset
//      if [[ "${RESULT}" != "Robot   True" ]]; then
//        RESULT="Robot   Error: Reset failed"
//      else
//        RESULT="Robot   OK: Resetting"
//        echo -e "${RESULT}"
//        RESULT="Robot   Run ..."
//        echo -e "${RESULT}"
//        ${SSH_COMM} "cd robot/brain; git pull; git checkout ${DEV_BRANCH} && git stash; git stash drop; git pull && \
//          RUST_LOG=info target/arm-unknown-linux-gnueabihf/debug/brain live setup.yaml
//          "
//        EXIT=$?
//        if [[ $EXIT -eq 0 ]]; then
//          RESULT="Robot   True"
//        else
//          RESULT="Robot   Error: "$EXIT
//        fi
//      fi
//    fi
//  fi
}

fn compile() {
    let output = Command::new("cross")
        .arg("build")
        .arg("--target=arm-unknown-linux-gnueabihf")
        .current_dir(BRAIN_DIR)
        .output()
        .expect("cross compile command failed to start");
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("{}", stdout);
}
