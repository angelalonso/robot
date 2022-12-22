use circuits::status_node::*;
use circuits::test_node::*;
//use circuits::action_client_node::*;
use circuits::led_action_server_node::*;

use std::collections::HashMap;
use std::thread;

#[tokio::main]
async fn main() {
    let mut node_server_action_led = LedActionServerNode::new(
        "led_action_server",
        get_conns(["led_action_server", "status", "test"].to_vec()),
    );
    //let mut node_status = StatusNode::new("8002", "8003");
    let mut node_test = TestNode::new("8002", "8101");

    let handle_led = thread::spawn(move || {
        node_server_action_led.receive();
    });
    //let handle_st = thread::spawn(move || {
    //    node_status.receive();
    //});
    let handle_ts = thread::spawn(move || {
        node_test.receive();
    });

    handle_led.join().unwrap();
    //handle_st.join().unwrap();
    handle_ts.join().unwrap();
}

fn get_conns(names: Vec<&str>) -> HashMap<&str, circuits::led_action_server_node::Conn> {
    // build connection list in a hashmap
    let mut all_conns: HashMap<&str, Conn> = HashMap::new();
    all_conns.insert(
        "led_action_server",
        Conn {
            port_in: "8101",
            port_out: "8002",
        },
    );
    all_conns.insert(
        "status",
        Conn {
            port_in: "8002",
            port_out: "8003",
        },
    );
    all_conns.insert(
        "test",
        Conn {
            port_in: "8003",
            port_out: "8101",
        },
    );
    // return only the ones needed
    let result: HashMap<&str, Conn> = all_conns
        .into_iter()
        .filter(|(k, _v)| names.contains(&k))
        .collect();
    return result;
}
