use circuits::action_client_node::*;
use circuits::action_server_node::*;

use std::thread;

#[tokio::main]
async fn main() {
    //let mut node_port: HashMap<String, String> = HashMap::new();
    // TODO: add infos in a hashmap

    let node_action_server_led = ActionServerNode::new("8401", "8501");
    let mut node_action_client_led = ActionClientNode::new("8501", "8401");

    let handle_s = thread::spawn(move || {
        node_action_server_led.run();
    });
    let handle_c = thread::spawn(move || {
        node_action_client_led.run();
    });
    handle_s.join().unwrap();
    handle_c.join().unwrap();
}
