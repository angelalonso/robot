pub mod api_node;
pub mod comms;
pub mod gpio_robot;
pub mod led_action_server_node;
pub mod mock_rust_pigpio;
pub mod motor_l_action_server_node;
pub mod motor_r_action_server_node;
pub mod status_node;
pub mod test_node;

use std::collections::HashMap;

pub fn get_conns(names: Vec<&str>) -> HashMap<&str, &str> {
    // build connection list in a hashmap
    let mut all_conns: HashMap<&str, &str> = HashMap::new();
    all_conns.insert("led", "8101");
    all_conns.insert("motor_l", "8102");
    all_conns.insert("motor_r", "8103");
    all_conns.insert("api", "8202");
    all_conns.insert("status", "8201");
    all_conns.insert("test", "9000");
    // return only the ones needed
    let result: HashMap<&str, &str> = all_conns
        .into_iter()
        .filter(|(k, _v)| names.contains(k))
        .collect();
    result
}
