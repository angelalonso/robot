
#[cfg(test)]
mod brain_test {
    use std::process;
    use crate::brain::Brain;

    #[test]
    fn check_actions() {
        let mut test_brain = Brain::new("Test Brain".to_string(), 
                                        "dryrun".to_string(), 
                                        "testfiles/cfg.yaml".to_string()
                                        ).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        // loop with timestamp 
        for counter in 0..1000 {
            let timestamp = counter as f64 / 100 as f64;
            let actions = test_brain.get_actions_from_rules();
            //assert_eq!(actions, expected_actions[counter]);
            println!("{} {:#x?}", timestamp, actions);
        }
        //let action_got = test.get_brain_actions("unexistingping\r\n");
        //assert!(action_got.is_err(), "getting an error for a non existing action did not go well");
    }
}
