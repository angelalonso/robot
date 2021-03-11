extern crate clap;


fn main() {
    use clap::{load_yaml, App};

    // To load a yaml file containing our CLI definition such as the example '17_yaml.yaml' we can
    // use the convenience macro which loads the file at compile relative to the current file
    // similar to how modules are found.
    //
    // Then we pass that yaml object to App to build the CLI.
    //
    // Finally we call get_matches() to start the parsing process. We use the matches just as we
    // normally would
    let yaml = load_yaml!("menu.yaml");
    let m = App::from(yaml).get_matches();

    // Because the example 17_yaml.yaml is rather large we'll just look a single arg so you can
    // see that it works...
    println!("verb: {:?}", m.value_of("verb"));
    println!("verb_do: {:?}", m.value_of("verb_do"));
    println!("verb_get: {:?}", m.value_of("verb_get"));
    println!("predicate: {:?}", m.value_of("predicate"));
}

    //let matches = App::new(option_env!("CARGO_PKG_NAME").unwrap())
    //                      .version(option_env!("CARGO_PKG_VERSION").unwrap())
    //                      .author(option_env!("CARGO_PKG_AUTHORS").unwrap())
    //                      .about("controls your robot")
    //                      .arg(Arg::with_name("VERB")
    //                           .help("Controls your workflow with the robot")
    //                           .required(true)
    //                           .takes_value(true)
	//						   .possible_value("get")
	//						   .possible_value("do")
    //                           .index(1))
    //                      .arg(Arg::with_name("GET_SMTHG")
    //                           .help("Sets the predicate to complement the verb")
    //                           .required(false)
	//						   .possible_value("online")
    //                           .index(2))
    //                      .arg(Arg::with_name("DO_SMTHG")
    //                           .help("Sets the predicate to complement the verb")
    //                           .required(false)
	//						   .possible_value("check")
	//						   .possible_value("run")
	//						   .possible_value("record")
	//						   .possible_value("reset")
	//						   .possible_value("test")
	//						   .possible_value("compile")
	//						   .possible_value("gitpush")
    //                           .index(2))
    //                      .get_matches();
