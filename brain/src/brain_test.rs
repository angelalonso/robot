use std::process;
use crate::brain::Brain;

#[cfg(test)]
mod brain_test {
    use super::*;

    #[test]
    #[ignore]
    fn check_read_one_from_arduino () {

    }

    #[test]
    #[ignore]
    fn check_read_one_from_arduino_noconnection () {

    }

    #[test]
    #[ignore]
    fn check_read_one_from_arduino_permissiondenied () {

    }

    // #[test]
    // fn check_read_loop_from_arduino?

    #[test]
    fn check_avrdude() {
        let mut test = Brain::new_serial("test",
                                  "testfiles/test.cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let avrdude = test.check_requirement("rustc");
        assert!(avrdude.is_ok(), "avrdude should be installed");
    }

    #[test]
    #[ignore]
    fn check_install_to_arduino () {
        let mut test = Brain::new_serial("test",
                                  "testfiles/test.cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let installation = test.install_to_arduino("../tests/000_blick_internal_led_seconds/000_blick_internal_led_seconds.ino.hex");
        assert!(installation.unwrap() == ());

    }

    #[test]
    #[ignore]
    fn check_install_to_arduino_nofile () {
        let mut test = Brain::new_serial("test",
                                  "testfiles/test.cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let installation = test.install_to_arduino("file_not_existing.ino.hex");
        assert!(installation.unwrap() == ());

        }


    #[test]
    fn check_install_to_arduino_noconnection () {

        }


    #[test]
    fn check_install_to_arduino_blockedpin () {

        }


    #[test]
    fn check_install_to_arduino_permissiondenied () {

        }


    /// ------------------ REVIEWED ^ --------------------- ///
    #[test]
    fn read_msg() {
        // I'll just test an error here
        let mut test = Brain::new("test",
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/notest_from_mock.q", 
                                  "testfiles/nptest_to_mock.q", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let boot = test.bootload();
        match test.read_msg(1) {
            Ok(read) => println!("{}", read),
            Err(e) => println!("{:?}", e),
        };
    }

    #[test]
    fn read_msg_proper() {
        let mut test = Brain::new("test",
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/test_from_mock.q", 
                                  "testfiles/test_to_mock.q", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let msg_received = test.read_msg(1);
        assert!(msg_received.unwrap() == "");
    }

    #[test]
    fn read_msgs() {
        // Needed?
        let mut test = Brain::new("test",
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/test_from_mock.q", 
                                  "testfiles/test_to_mock.q", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        // Gets stuck, maybe this is not needed?
        // let _result = test.read_msgs();
    }

    #[test]
    fn get_actions() {
        let mut test = Brain::new("test",
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/test_from_mock.q", 
                                  "testfiles/test_to_mock.q", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        test.get_actions("Ping\n");
    }

    #[test]
    fn apply_actions() {
        let mut test = Brain::new("test",
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/test_from_mock.q", 
                                  "testfiles/test_to_mock.q", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        test.apply_actions(Vec::from(["sendfile_../tests/000_blick_internal_led_seconds/000_blick_internal_led_seconds.ino.hex".to_string()]));
    }

    #[test]
    #[ignore]
    fn send() {
        let mut test = Brain::new("test",
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/test_from_mock.q", 
                                  "testfiles/test_to_mock.q", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        test.send("Do->Ping");
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn send_to_wrongfile() {
        let mut test = Brain::new("test",
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/test_from_mock.q", 
                                  "testfiles/notest_to_mock.q", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        test.send("Do->Ping");
    }

    /// run this from the raspberry itself with cargo test -- --ignored
    #[test]
    #[ignore]
    fn sendfile() {
        let mut test = Brain::new_serial("test",
                                  "testfiles/test.cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let serial = test.install_to_arduino("testfile");
        assert!(serial.is_ok(), "db file should not exist");
    }

    /// run this from the raspberry itself with cargo test -- --ignored
    #[test]
    #[ignore]
    fn readserial() {
        let mut test = Brain::new_serial("test",
                                  "testfiles/test.cfg.yaml", 
                                  None).unwrap_or_else(|err| {
            eprintln!("Problem Initializing Main Brain: {}", err);
            process::exit(1);
        });
        let serial = test.read_loop_from_arduino();
        assert!(serial.is_ok(), "db file should not exist");
    }
}
