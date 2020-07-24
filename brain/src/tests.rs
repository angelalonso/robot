use crate::comm::Messages;

// TODO: convention is to
// "put unit tests in the src directory in each file with the code that they're testing."
// "The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test) "

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_rx_msgs() {
        let mut test = Messages::new();
        test.add(true, "a test message".to_string());
        test.add(true, "another test message".to_string());
        let list = test.get_list("received");
        match list {
                Some(l) => assert_eq!(vec!["a test message", "another test message"], l),
                None => println!("Error"),
            }
    }
    #[test]
    fn store_tx_msgs() {
        let mut test = Messages::new();
        test.add(false, "a test message".to_string());
        test.add(false, "another test message".to_string());
        let list = test.get_list("transmitted");
        match list {
                Some(l) => assert_eq!(vec!["a test message", "another test message"], l),
                None => println!("Error"),
            }
    }
    #[test]
    fn listen() {
        let mut test = Messages::new();
        let read_expected: String = String::from("Test\n");
        let _read_result = match test.read_the_buffer_mock(){
            Ok(result) => assert_eq!(read_expected, result),
            _ => panic!("{:?}", "ERROR"),
        };
    }
}
