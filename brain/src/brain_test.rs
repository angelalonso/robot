
#[cfg(test)]
mod brain_test {
    use std::process;
    use crate::brain::Brain;
    use std::fs::File;
    use std::sync::mpsc::{Sender, Receiver};
    use std::thread;

    extern crate serde_yaml;

    #[derive(Clone, Debug, PartialEq, Deserialize)]
    pub struct ActionEntry {
        time: f64,
        action: String,
    }

    #[test]
    fn check_actions() {
        let expected_pointer = File::open("testfiles/actions.yaml").unwrap();
        let _e: Vec<ActionEntry> = serde_yaml::from_reader(expected_pointer).unwrap();
        let mut test_brain = Brain::new("Test Brain".to_string(), 
                                        "dryrun".to_string(), 
                                        "testfiles/cfg.yaml".to_string()
                                        ).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        // loop with timestamp 
        let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let handle = thread::spawn(move || {
            let _actions = test_brain.run(Some(1.4), 10, s);
        });
        handle.join().unwrap();
        loop {
            let msg = match r.try_recv() {
                Ok(m) => println!("{:?}", m),
                Err(_) => break,
            };
        }
        //assert_eq!(actions, expected_actions[counter]);
        //let action_got = test.get_brain_actions("unexistingping\r\n");
        //assert!(action_got.is_err(), "getting an error for a non existing action did not go well");
    }
}
