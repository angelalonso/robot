use docopt::Docopt;
use serde::{Deserialize, Serialize};
use serde_yaml;

const USAGE: &'static str = "
Robot controller.

Usage:
  roctl get (online|status)
  roctl do (run|test|reset)
  roctl init
  roctl (-h | --help)
  roctl --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Args {
    cmd_get: bool,
    cmd_online: bool,
    cmd_status: bool,
    cmd_do: bool,
    cmd_run: bool,
    cmd_test: bool,
    cmd_reset: bool,
    cmd_init: bool,
}

fn main() {
    let mut mode: String = "".to_owned();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    // This does not look very elegant but somehow...it's short at least
    if args.cmd_get {
        if args.cmd_online { mode.push_str("get_online")
        } else if args.cmd_status { mode.push_str("get_status")
        };
    } else if args.cmd_do {
        if args.cmd_run { mode.push_str("do_run")
        } else if args.cmd_test { mode.push_str("do_test")
        } else if args.cmd_reset { mode.push_str("do_reset")
        };
    } else if args.cmd_init {mode.push_str("init")};

    println!("{}", mode);

}
