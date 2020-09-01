
#[cfg(test)]
mod arduino_test {
    use std::process;
    use crate::arduino::Arduino;

    #[test]
    #[ignore]
    fn check_read_channel() {

    }

    #[test]
    #[ignore]
    fn check_interact() {

    }

    #[test]
    #[ignore]
    fn check_install() {
        let mut test = Arduino::new("testduino", None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let serial = test.install("../arduino/001_test_ping/001_test_ping.ino.hex");
        assert!(serial.is_ok(), "installing file did not work well");
    }
    #[test]
    fn check_install_nofile () {
        //let mut test = Brain::new("test",
        //                          "testfiles/cfg.yaml", 
        //                          None).unwrap_or_else(|err| {
        //    eprintln!("Problem Initializing Main Brain: {}", err);
        //    process::exit(1);
        //});
        let mut test = Arduino::new("testduino", None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let serial = test.install("file_not_existing.ino.hex");
        assert!(serial.is_err(), "checking errors on installing wrong file did not work well");
    }

    // TODO: fill this up
    #[test]
    fn check_install_noconnection () {

    }
    // TODO: fill this up
    #[test]
    fn check_install_lockedpin () {

    }
    // TODO: fill this up
    #[test]
    fn check_install_ermissiondenied () {

    }

    #[test]
    fn check_check_requirement() {
        let mut test = Arduino::new("testduino", None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let avrdude = test.check_requirement("rustc");
        assert!(avrdude.is_ok(), "avrdude should be installed");
    }
    #[test]
    #[ignore]
    fn check_avrdude() {
        let mut test = Arduino::new("testduino", None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let avrdude = test.check_requirement("avrdude");
        assert!(avrdude.is_ok(), "avrdude should be installed");
    }
}
