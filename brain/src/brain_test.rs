
#[cfg(test)]
mod brain_test {
    use std::process;
    use crate::brain::Brain;

    /// Ignoring this outside of the raspberry because it goes on to use avrdude
    #[test]
    #[ignore]
    fn check_get_actions_notfound() {
        let mut test = Brain::new("test".to_string(),
                                  "testfiles/cfg.yaml".to_string(), 
                                  "".to_string()).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        //let action_got = test.get_brain_actions("unexistingping\r\n");
        //assert!(action_got.is_err(), "getting an error for a non existing action did not go well");
    }
}
