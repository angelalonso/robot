use std::process::Command;
use std::env;

pub fn run() {
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
}

fn compile() -> bool {
    let cwd: String = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let brain_dir: &str = &format!("{}/../brain/", cwd);
    // TODO: avoid using a script to do this. Is it possible even?
    let comm_status = Command::new("./roctl_do_compile.sh")
        .current_dir(brain_dir)
        .status()
        .expect("cross compile command failed to start");
    if comm_status.success() {
        return true
            
    } else {
        return false
    }
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

fn reset() -> bool {
    true
}
