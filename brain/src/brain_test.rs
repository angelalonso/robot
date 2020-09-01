
#[cfg(test)]
mod brain_test {
    use std::process;
    use crate::brain::Brain;

    #[test]
    #[ignore]
    fn check_read() {
    }

    /// Ignoring this outside of the raspberry because it goes on to use avrdude
    #[test]
    #[ignore]
    fn check_get_actions() {
        let mut test = Brain::new("test",
                                  "testfiles/cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let action_got = test.get_actions("ping\r\n");
        assert!(action_got.is_ok(), "getting an action did not go well");
    }
    #[test]
    fn check_get_actions_notfound() {
        let mut test = Brain::new("test",
                                  "testfiles/cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let action_got = test.get_actions("unexistingping\r\n");
        assert!(action_got.is_err(), "getting an error for a non existing action did not go well");
    }

    #[test]
    fn check_apply_actions() {
        let mut test = Brain::new("test",
                                  "testfiles/cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let action_applied = test.apply_actions(Vec::from(["sendfile_../tests/000_blick_internal_led_seconds/000_blick_internal_led_seconds.ino.hex".to_string()]));
        assert!(action_applied.is_ok(), "applying an action did not go well");
    }
    #[test]
    fn check_apply_actions_error() {
    }

    #[test]
    fn check_do_nothing() {
        let mut test = Brain::new("test",
                                  "testfiles/cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let avrdude = test.do_nothing();
        assert!(avrdude.is_ok(), "nothing is being done");
    }
}
