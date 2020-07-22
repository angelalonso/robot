pub mod comm;
pub mod action;

pub mod pollwatch;
// pub mod tailwatch;

pub use pollwatch::PollWatcher;
// pub use tailwatch::TailWatcher;

pub trait Watcher<'a> {
    fn new(filename: &str, period_milliseconds: u64) -> Self;
    fn register(&mut self, callback: Box<dyn FnMut(String) + 'a>);
    fn watch(&mut self);
    fn watch_once(&mut self);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_rx_msgs() {
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
    fn store_tx_msgs() {
        let mut test = comm::Messages::new();
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
        let mut test = comm::Messages::new();
        let read_expected: String = String::from("Test");
        let read_result = match test.read_the_buffer_on_test(){
            Ok(result) => assert_eq!(read_expected, result),
            _ => panic!("{:?}", "ERROR"),
        };
    }
}
