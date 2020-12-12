// TODO: test proximity actionset
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
    #[ignore]
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
        for (ix, e_raw) in expected.iter().enumerate() {
            let e = e_raw.split("|").collect::<Vec<_>>();
            let g = got[ix].split("|").collect::<Vec<_>>();
            println!("- expected: {:#x?} \n- got: {:#x?}", e[0], g[0]);
            assert_eq!(e[0], g[0]);
            let e_str = e[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut e_vec = e_str.split(", ").collect::<Vec<_>>();
            let g_str = g[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut g_vec = g_str.split(", ").collect::<Vec<_>>();
            e_vec.sort();
            g_vec.sort();
            println!("  - expected: {:?} \n  - got: {:?}", e_vec, g_vec);
            assert_eq!(e_vec, g_vec);
        }
    }

    #[test]
    #[ignore]
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
        for (ix, e_raw) in expected.iter().enumerate() {
            let e = e_raw.split("|").collect::<Vec<_>>();
            let g = got[ix].split("|").collect::<Vec<_>>();
            println!("- expected: {:#x?} \n- got: {:#x?}", e[0], g[0]);
            assert_eq!(e[0], g[0]);
            let e_str = e[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut e_vec = e_str.split(", ").collect::<Vec<_>>();
            let g_str = g[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut g_vec = g_str.split(", ").collect::<Vec<_>>();
            e_vec.sort();
            g_vec.sort();
            println!("  - expected: {:?} \n  - got: {:?}", e_vec, g_vec);
            assert_eq!(e_vec, g_vec);
        }
    }

    // TODO: order on vector should be irrelevant
    #[test]
    #[ignore]
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
        for (ix, e_raw) in expected.iter().enumerate() {
            let e = e_raw.split("|").collect::<Vec<_>>();
            let g = got[ix].split("|").collect::<Vec<_>>();
            println!("- expected: {:#x?} \n- got: {:#x?}", e[0], g[0]);
            assert_eq!(e[0], g[0]);
            let e_str = e[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut e_vec = e_str.split(", ").collect::<Vec<_>>();
            let g_str = g[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut g_vec = g_str.split(", ").collect::<Vec<_>>();
            e_vec.sort();
            g_vec.sort();
            println!("  - expected: {:?} \n  - got: {:?}", e_vec, g_vec);
            assert_eq!(e_vec, g_vec);
        }
    }

    #[test]
    #[ignore]
    fn check_actions_button() {
        let expected_pointer = File::open("testfiles/button_expected.yaml").unwrap();
        let e: Vec<ActionEntry> = serde_yaml::from_reader(expected_pointer).unwrap();
        let mut expected = [].to_vec();
        for entry in e{
            expected.push(format!("{:?}|{:?}", entry.time, entry.actions));
        }

        let mut test_brain = Brain::new("Test Brain".to_string(), 
                                        "dryrun".to_string(), 
                                        "testfiles/button_setup.yaml".to_string()
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
        for (ix, e_raw) in expected.iter().enumerate() {
            let e = e_raw.split("|").collect::<Vec<_>>();
            let g = got[ix].split("|").collect::<Vec<_>>();
            println!("- expected: {:#x?} \n- got: {:#x?}", e[0], g[0]);
            assert_eq!(e[0], g[0]);
            let e_str = e[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut e_vec = e_str.split(", ").collect::<Vec<_>>();
            let g_str = g[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut g_vec = g_str.split(", ").collect::<Vec<_>>();
            e_vec.sort();
            g_vec.sort();
            println!("  - expected: {:?} \n  - got: {:?}", e_vec, g_vec);
            assert_eq!(e_vec, g_vec);
        }
    }

    #[test]
    #[ignore]
    fn check_actions_obstacle1() {
        let expected_pointer = File::open("testfiles/obstacle1_expected.yaml").unwrap();
        let e: Vec<ActionEntry> = serde_yaml::from_reader(expected_pointer).unwrap();
        let mut expected = [].to_vec();
        for entry in e{
            expected.push(format!("{:?}|{:?}", entry.time, entry.actions));
        }

        let mut test_brain = Brain::new("Test Brain".to_string(), 
                                        "dryrun".to_string(), 
                                        "testfiles/obstacle1_setup.yaml".to_string()
                                        ).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        // loop with timestamp 
        let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let handle = thread::spawn(move || {
            let _actions = test_brain.run(Some(0.9), 100, s);
        });
        handle.join().unwrap();
        let mut got = [].to_vec();
        loop {
            match r.try_recv() {
                Ok(m) => got.push(m),
                Err(_) => break,
            };
        }
        for (ix, e_raw) in expected.iter().enumerate() {
            let e = e_raw.split("|").collect::<Vec<_>>();
            let g = got[ix].split("|").collect::<Vec<_>>();
            println!("- expected: {:#x?} \n- got: {:#x?}", e[0], g[0]);
            assert_eq!(e[0], g[0]);
            let e_str = e[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut e_vec = e_str.split(", ").collect::<Vec<_>>();
            let g_str = g[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut g_vec = g_str.split(", ").collect::<Vec<_>>();
            e_vec.sort();
            g_vec.sort();
            println!("  - expected: {:?} \n  - got: {:?}", e_vec, g_vec);
            assert_eq!(e_vec, g_vec);
        }
    }

    #[test]
    fn check_actions_obstacle2() {
        let expected_pointer = File::open("testfiles/obstacle2_expected.yaml").unwrap();
        let e: Vec<ActionEntry> = serde_yaml::from_reader(expected_pointer).unwrap();
        let mut expected = [].to_vec();
        for entry in e{
            expected.push(format!("{:?}|{:?}", entry.time, entry.actions));
        }

        let mut test_brain = Brain::new("Test Brain".to_string(), 
                                        "dryrun".to_string(), 
                                        "testfiles/obstacle2_setup.yaml".to_string()
                                        ).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        // loop with timestamp 
        let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
        let handle = thread::spawn(move || {
            let _actions = test_brain.run(Some(0.9), 100, s);
        });
        handle.join().unwrap();
        let mut got = [].to_vec();
        loop {
            match r.try_recv() {
                Ok(m) => got.push(m),
                Err(_) => break,
            };
        }
        for (ix, e_raw) in expected.iter().enumerate() {
            let e = e_raw.split("|").collect::<Vec<_>>();
            let g = got[ix].split("|").collect::<Vec<_>>();
            println!("- expected: {:#x?} \n- got: {:#x?}", e[0], g[0]);
            assert_eq!(e[0], g[0]);
            let e_str = e[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut e_vec = e_str.split(", ").collect::<Vec<_>>();
            let g_str = g[1].to_string().replace(&['[', ']', '\"'][..], "");
            let mut g_vec = g_str.split(", ").collect::<Vec<_>>();
            e_vec.sort();
            g_vec.sort();
            println!("  - expected: {:?} \n  - got: {:?}", e_vec, g_vec);
            assert_eq!(e_vec, g_vec);
        }
    }

}
