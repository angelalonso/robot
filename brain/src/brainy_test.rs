use crate::brain::Brain;
use std::io;
use std::thread;

#[cfg(test)]
mod brainy_test {
    use super::*;

    #[test]
    fn read_msg() {
        // I'll just test an error here
        let mut test = Brain::new("test", 
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/notest_from_mock.q", 
                                  "testfiles/nptest_to_mock.q", 
                                  None);
        test.bootload();
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
                                  None);
        let msg_received = test.read_msg(1);
        assert!(msg_received.unwrap() == "");
    }

    #[test]
    fn read_msgs() {
        // I'll just test an error here
        let mut test = Brain::new("test", 
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/notest_from_mock.q", 
                                  "testfiles/nptest_to_mock.q", 
                                  None);
        let _result = test.read_msgs().unwrap();
    }

    #[test]
    fn get_actions() {
        let mut test = Brain::new("test", 
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/test_from_mock.q", 
                                  "testfiles/test_to_mock.q", 
                                  None);
        test.get_actions("Ping\n");
    }

    #[test]
    fn apply_actions() {
        let mut test = Brain::new("test", 
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/test_from_mock.q", 
                                  "testfiles/test_to_mock.q", 
                                  None);
        test.apply_actions(Vec::from(["send_ping".to_string()]));
    }

    #[test]
    fn send() {
        let mut test = Brain::new("test", 
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/test_from_mock.q", 
                                  "testfiles/test_to_mock.q", 
                                  None);
        test.send("Do->Ping");
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn send_to_wrongfile() {
        let mut test = Brain::new("test", 
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/test_from_mock.q", 
                                  "testfiles/notest_to_mock.q", 
                                  None);
        test.send("Do->Ping");
    }

    #[test]
    fn sendfileserial() {
        let mut test = Brain::new("test", 
                                  "testfiles/test.cfg.yaml", 
                                  "testfiles/test_from_mock.q", 
                                  "testfiles/test_to_mock.q", 
                                  None);
        test.sendfileserial("testfile");

    }
}
