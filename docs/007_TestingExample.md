# How to do the testing of a new ruleset

The process guides you to creating and using 4 files:
- The ruleset/config yaml that defines what the robot should do given certain conditions (`*_rules.yaml`). Remember that this is the file you want to test.
- A mock yaml including signals received from the arduino (`*_mock.yaml`).
- A setup yaml that defines how the brain starts (`*_setup.yaml`).
- An expectations yaml that defines what actions your robot should do and when (`*_expected.yaml`).

Normally you'd want all these files (with * being the same word of your choice for all of them) under ./testfiles. Then once you are happy with the result, you can just copy your *_rules.yaml to the rulesets/ folder.  

## Create your ruleset/config file (or copy one)
Make sure the file is called `testfiles/<same name of your choice>_rules.yaml`.

## Get data from your robot to mock inputs
```
./do_record.sh <filename>
```

, where you probably want to make your filename `testfiles/<name of your choice>_mock.yaml`. Otherwise you'll have to rename it to that, because the path and name are important!

## Create your setup file (or copy one)
Make sure the file is called `testfiles/<same name of your choice>_setup.yaml`, and you configure it properly like:
```
start_actionset_file: testfiles/<same name of your choice>_rules.yaml
```

## Create your expectations file (or copy one)
You can copy testfiles/expected_template.yaml over and check around for the format of expected actions.
Make sure the file is called `testfiles/<same name of your choice>_expected.yaml`.

## Add another test function to src/brain_test.rs
Again, copy and adapt examples on that file, they are straightforward.

Here goes an example, though, for the code to test a new function called obstacle1:
```
    #[test]
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
            let _actions = test_brain.run(Some(7.9), 10, s);
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
```

## TEST!
```
cargo test -- --nocapture
```

If you want something closer to a live run, you can run brain on test mode too:
- Create (or copy over) a setup_test.yaml file
- Create (or copy over) a mock_test.yaml file
- Run:
```
clear && RUST_LOG=info cargo run test setup_test.yaml
```
, you can even play with the RUST_LOG variable or change the code to just publish some stuff you want to debug. Feel free to do so as long as you can return to the original code somehow ;).
