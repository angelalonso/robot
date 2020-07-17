pub mod comm;
pub mod action;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receive_msgs() {
        let mut test = comm::Messages::new();
        test.add(true, "a test message".to_string());
        test.add(true, "another test message".to_string());
        let list = test.get_list("received");
        match list {
                Some(l) => assert_eq!(vec!["a test message", "another test message"], l),
                None => println!("Error"),
            }
    }
    #[test]
    fn transmit_msgs() {
        let mut test = comm::Messages::new();
        test.add(false, "a test message".to_string());
        test.add(false, "another test message".to_string());
        let list = test.get_list("transmitted");
        match list {
                Some(l) => assert_eq!(vec!["a test message", "another test message"], l),
                None => println!("Error"),
            }
    }
}
