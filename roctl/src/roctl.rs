pub fn proxy_action(mode: String) {
    let mode_split: Vec<&str> = mode.split("_").collect();
    match mode_split[0] {
        "init" => {
            if mode.replace("init", "") != "" {
                println!("ERROR");
            } else {init()};
        },
        "get" => {
            let mode_clean = mode.replace("get_", "");
            if mode_clean == "" {
                println!("ERROR");
            } else {get_stuff(mode_clean)};
        },
        "do" => {
            let mode_clean = mode.replace("do_", "");
            if mode_clean == "" {
                println!("ERROR");
            } else {do_stuff(mode_clean)};
        },
        &_ => println!("ERROR"),
    };
}

fn init() {
    println!("INIT mode");
}

fn get_stuff(what: String) {
    println!("GET mode with {} parameters", what);
}

fn do_stuff(what: String) {
    println!("DO mode with {} parameters", what);
}
