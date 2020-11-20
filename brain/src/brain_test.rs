// TODO: Test the following failing cases:
// - outputs with repeat and wait and load_x dont repeat
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
        actions: Vec<String>,
    }

    #[test]
    fn check_actions_simple() {
        let expected_pointer = File::open("testfiles/simple_expected.yaml").unwrap();
        let e: Vec<ActionEntry> = serde_yaml::from_reader(expected_pointer).unwrap();
        let mut expected = [].to_vec();
        for entry in e{
            expected.push(format!("{:?}|{:?}", entry.time, entry.actions));
        }

        let mut test_brain = Brain::new("Test Brain".to_string(), 
                                        "dryrun".to_string(), 
                                        "testfiles/simple_setup.yaml".to_string()
                                        ).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        // loop with timestamp 
        let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let handle = thread::spawn(move || {
            let _actions = test_brain.run(Some(1.1), 10, s);
        });
        handle.join().unwrap();
        let mut got = [].to_vec();
        loop {
            match r.try_recv() {
                Ok(m) => got.push(m),
                Err(_) => break,
            };
        }
        println!("expected: {:#x?}", expected);
        println!("got: {:#x?}", got);
        assert_eq!(got, expected);
    }

    #[test]
    fn check_actions_looplights() {
        let expected_pointer = File::open("testfiles/looplights_expected.yaml").unwrap();
        let e: Vec<ActionEntry> = serde_yaml::from_reader(expected_pointer).unwrap();
        let mut expected = [].to_vec();
        for entry in e{
            expected.push(format!("{:?}|{:?}", entry.time, entry.actions));
        }

        let mut test_brain = Brain::new("Test Brain".to_string(), 
                                        "dryrun".to_string(), 
                                        "testfiles/looplights_setup.yaml".to_string()
                                        ).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        // loop with timestamp 
        let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let handle = thread::spawn(move || {
            let _actions = test_brain.run(Some(1.8), 10, s);
        });
        handle.join().unwrap();
        let mut got = [].to_vec();
        loop {
            match r.try_recv() {
                Ok(m) => got.push(m),
                Err(_) => break,
            };
        }
        println!("expected: {:#x?}", expected);
        println!("got: {:#x?}", got);
        assert_eq!(got, expected);
    }

    #[test]
    fn check_actions_doublefile() {
        let expected_pointer = File::open("testfiles/doublefile_expected.yaml").unwrap();
        let e: Vec<ActionEntry> = serde_yaml::from_reader(expected_pointer).unwrap();
        let mut expected = [].to_vec();
        for entry in e{
            expected.push(format!("{:?}|{:?}", entry.time, entry.actions));
        }

        let mut test_brain = Brain::new("Test Brain".to_string(), 
                                        "dryrun".to_string(), 
                                        "testfiles/doublefile_setup.yaml".to_string()
                                        ).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        // loop with timestamp 
        let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let handle = thread::spawn(move || {
            let _actions = test_brain.run(Some(1.8), 10, s);
        });
        handle.join().unwrap();
        let mut got = [].to_vec();
        loop {
            match r.try_recv() {
                Ok(m) => got.push(m),
                Err(_) => break,
            };
        }
        println!("expected: {:#x?}", expected);
        println!("got: {:#x?}", got);
        assert_eq!(got, expected);
    }
}
