
#[cfg(test)]
mod arduino_test {
    use std::process;
    use crate::arduino::Arduino;

    // TODO: add a limit on loops before enabling this
    #[test]
    #[ignore]
    fn check_read_channel() {
    //    let mut test = Arduino::new("testduino", None).unwrap_or_else(|err| {
    //        eprintln!("Problem Initializing Main Brain: {}", err);
    //        process::exit(1);
    //    });
    //    let serial = test.read_channel();
    //    assert!(serial.is_ok(), "reading from the Serial Port did not work well");
    }

    #[test]
    #[ignore]
    fn check_interact() {
            
        let mut test = Arduino::new("testduino".to_string(), Some("/dev/ttyUSB0".to_string())).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let mut port = serial::open("/dev/ttyUSB0").unwrap();
        let serial = test.interact(&mut port);
        assert!(serial.is_ok(), "interacting with the Serial Port did not work well");
    }
    #[test]
    #[ignore]
    fn check_interact_error() {
        // Not needed because all we'd need to check would be:
        //   let wrong_port = serial::open("/dev/ttyNONE");
        //   assert!(wrong_port.is_err(), "interacting with the WRONG Serial Port did not return a proper error");
    }

    #[test]
    #[ignore]
    fn check_install() {
        let mut test = Arduino::new("testduino".to_string(), None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let serial = test.install("../arduino/001_test_ping/001_test_ping.ino.hex");
        assert!(serial.is_ok(), "installing file did not work well");
    }
    #[test]
    #[ignore]
    fn check_install_nofile () {
        let mut test = Arduino::new("testduino".to_string(), None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let serial = test.install("file_not_existing.ino.hex");
        assert!(serial.is_err(), "checking errors on installing wrong file did not work well");
    }
    #[test]
    fn check_install_noconnection () {
        let mut test = Arduino::new("testduino".to_string(), None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let serial = test.install("file_not_existing.ino.hex");
        assert!(serial.is_err(), "checking errors on installing without connection did not work well");
    }

    #[test]
    fn check_check_requirement() {
        let mut test = Arduino::new("testduino".to_string(), None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let avrdude = test.check_requirement("rustc");
        assert!(avrdude.is_ok(), "avrdude should be installed");
    }
    #[test]
    #[ignore]
    fn check_avrdude() {
        let mut test = Arduino::new("testduino".to_string(), None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let avrdude = test.check_requirement("avrdude");
        assert!(avrdude.is_ok(), "avrdude should be installed");
    }
}
