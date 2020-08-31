
#[cfg(test)]
mod brain_test {
    use std::process;
    use crate::brain::Brain;

    // TODO: fill this up
    #[test]
    fn check_read_loop() {

    }

    // TODO: fill this up
    #[test]
    fn check_read_one_from_serialport () {

    }
    // TODO: fill this up
    #[test]
    fn check_read_one_from_serialport_noconnection () {

    }
    // TODO: fill this up
    #[test]
    fn check_read_one_from_serialport_permissiondenied () {

    }

    /// Ignoring this outside of the raspberry because it goes on to use avrdude
    #[test]
    #[ignore]
    fn get_actions() {
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
    fn get_actions_notfound() {
        let mut test = Brain::new("test",
                                  "testfiles/cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let action_got = test.get_actions("unexistingping\r\n");
        assert!(action_got.is_err(), "getting an error for a non existing action did not go well");
    }

    // TODO: Add a failing one
    #[test]
    fn apply_actions() {
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
    #[ignore]
    fn check_install_to_arduino() {
        let mut test = Brain::new("test",
                                  "testfiles/cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let serial = test.install_to_arduino("../arduino/001_test_ping/001_test_ping.ino.hex");
        assert!(serial.is_ok(), "installing file did not work well");
    }
    #[test]
    fn check_install_to_arduino_nofile () {
        let mut test = Brain::new("test",
                                  "testfiles/cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let serial = test.install_to_arduino("file_not_existing.ino.hex");
        assert!(serial.is_err(), "checking errors on installing wrong file did not work well");
    }
    // TODO: fill this up
    #[test]
    fn check_install_to_arduino_noconnection () {

    }
    // TODO: fill this up
    #[test]
    fn check_install_to_arduino_blockedpin () {

    }
    // TODO: fill this up
    #[test]
    fn check_install_to_arduino_permissiondenied () {

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

    #[test]
    fn check_check_requirement() {
        let mut test = Brain::new("test",
                                  "testfiles/cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let avrdude = test.check_requirement("rustc");
        assert!(avrdude.is_ok(), "avrdude should be installed");
    }
    #[test]
    #[ignore]
    fn check_avrdude() {
        let mut test = Brain::new("test",
                                  "testfiles/cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let avrdude = test.check_requirement("avrdude");
        assert!(avrdude.is_ok(), "avrdude should be installed");
    }

//    /// ------------------ REVIEWED ^ --------------------- ///
//
//
//
//
//
//
//
//
//
}
