use std::process::Command;
use std::env;

pub fn run() -> bool {
    if compile() {
        println!("Compilation OK");
        if git_push() {
            println!("Pushing changes to branch OK");
            if reset() {
                println!("Resetting Robot OK");
            };
        };
    };
//  doCompile $@
//    doGitPush
//      doReset
//        ${SSH_COMM} "cd robot/brain; git pull; git checkout ${DEV_BRANCH} && git stash; git stash drop; git pull && \
    true
}

pub fn compile() -> bool {
    let log_mode = match env::var("RUST_LOG") {
        Ok(lm) => lm,
        Err(_) => "".to_string(),
    };
    let cwd: String = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let brain_dir: &str = &format!("{}/../brain/", cwd);
    // TODO: avoid using a script to do this. Is it possible even?
    match log_mode.as_str() {
        "debug" => {
            let comm_status = Command::new("./roctl_do_compile.sh")
                .current_dir(brain_dir)
                .status()
                .expect("cross compile command failed to start");
            if comm_status.success() {
                return true
            } else {
                return false
            }
        },
        &_ => {
            let comm_output = Command::new("./roctl_do_compile.sh")
                .current_dir(brain_dir)
                .output()
                .expect("cross compile command failed to start");
            if comm_output.status.success() {
                return true
            } else {
                return false
            }
        },

    };
}

fn git_push() -> bool {
    let cwd: String = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let brain_dir: &str = &format!("{}/../brain/", cwd);
    // TODO: use env vars to get branch 
    let gcheckout_status = Command::new("git")
        .arg("checkout")
        .arg("dev002")
        .current_dir(brain_dir)
        .status()
        .expect("git checkout command failed to start");
    if gcheckout_status.success() {
        let gadd_status = Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(brain_dir)
            .status()
            .expect("git add command failed to start");
        if gadd_status.success() {
            let gcommit_status = Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg("\"auto commit by roctl\"")
                .current_dir(brain_dir)
                .status()
                .expect("git commit command failed to start");
            if gcommit_status.success() {
                let gpush_status = Command::new("git")
                    .arg("push")
                    .arg("origin")
                    .arg("dev002")
                    .current_dir(brain_dir)
                    .status()
                    .expect("git push command failed to start");
                if gpush_status.success() {
                    return true
                } else {
                    return false
                }
            } else {
                return false
            }
        } else {
            return false
        }
    } else {
        return false
    }
}

pub fn reset() -> bool {
    let cwd: String = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let brain_dir: &str = &format!("{}/../brain/", cwd);
    // TODO: avoid using a script to do this. Is it possible even?
    let comm_output = Command::new("./roctl_do_compile.sh")
        .current_dir(brain_dir)
        .output()
        .expect("cross compile command failed to start");
    if comm_output.status.success() {
        return true
    } else {
        return false
    }

//    ${SSH_COMM} "kill \$(ps aux | grep brain | grep setup | awk '{print \$2}')" > /dev/null 2>&1
//    ${SSH_COMM} "cd robot/brain; \
//      RUST_LOG=info target/arm-unknown-linux-gnueabihf/debug/brain reset setup_reset.yaml
//      " > /dev/null 2>&1
//      EXIT=$?
//    if [[ $EXIT -eq 0 ]]; then
//      ${SSH_COMM} "kill \$(ps aux | grep brain | grep cfg | awk '{print \$2}')"
//      RESULT="Robot   True"
//    else
//      RESULT="Robot   Error: "$EXIT
//    fi
}

