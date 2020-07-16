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
        assert_eq!(vec!["a test message", "another test message"], test.get_list("received"));
    }
    #[test]
    fn transmit_msgs() {
        let mut test = comm::Messages::new();
        test.add(false, "a test message".to_string());
        test.add(false, "another test message".to_string());
        assert_eq!(vec!["a test message", "another test message"], test.get_list("transmitted"));
    }
}
