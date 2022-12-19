use circuits::comms::*;
use circuits::status_node::*;
use circuits::test_node::*;
//use circuits::action_client_node::*;
//use circuits::action_server_node::*;

use std::thread;

#[tokio::main]
async fn main() {
    //let mut node_port: HashMap<String, String> = HashMap::new();
    // TODO: add infos in a hashmap

    let mut node_status = StatusNode::new("8001", "8002");
    let mut node_test = TestNode::new("8002", "8001");
    //let node_action_server_led = ActionServerNode::new("8401", "8501");
    //let mut node_action_client_led = ActionClientNode::new("8501", "8401");

    let handle_st = thread::spawn(move || {
        //node_status.listen();
        node_status.receive();
    });
    let handle_ts = thread::spawn(move || {
        node_test.receive();
    });
    //let handle_s = thread::spawn(move || {
    //    node_action_server_led.run();
    //});
    //let handle_c = thread::spawn(move || {
    //    node_action_client_led.run();
    //});
    handle_st.join().unwrap();
    handle_ts.join().unwrap();
    //handle_s.join().unwrap();
    //handle_c.join().unwrap();
    //-//let mut t = UDPConn::new("55001".to_owned(), "55002".to_owned());
    //-//t.run();
}
