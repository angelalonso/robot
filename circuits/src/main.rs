use circuits::led_action_server_node::*;
use circuits::motor_l_action_server_node::*;
use circuits::motor_r_action_server_node::*;
use circuits::status_node::*;
use circuits::test_node::*;

use chrono::Local;
use env_logger::Builder;
use log::info;
use std::collections::HashMap;
use std::io::Write;
use std::thread;

#[tokio::main]
async fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y%m%dT%H:%M:%S%.3f"),
                record.level(),
                record.args(),
            )
        })
        .filter(None, log::LevelFilter::Debug)
        .init();

    let mut node_server_action_led = LedActionServerNode::new(
        "led_action_server",
        get_conns(["led_action_server", "status", "test"].to_vec()),
    );
    let mut node_server_action_motor_l = MotorLActionServerNode::new(
        "motor_l_action_server",
        get_conns(["motor_l_action_server", "status", "test"].to_vec()),
    );
    let mut node_server_action_motor_r = MotorLActionServerNode::new(
        "motor_r_action_server",
        get_conns(["motor_r_action_server", "status", "test"].to_vec()),
    );
    let mut node_status = StatusNode::new(
        "status",
        get_conns(
            [
                "motor_l_action_server",
                "led_action_server",
                "status",
                "test",
            ]
            .to_vec(),
        ),
    );
    let mut node_test = TestNode::new(
        "test",
        get_conns(
            [
                "motor_l_action_server",
                "led_action_server",
                "status",
                "test",
            ]
            .to_vec(),
        ),
    );

    info!("Led - Process started");
    std::thread::sleep(std::time::Duration::from_millis(50));
    let handle_st = thread::spawn(move || {
        node_status.talk();
    });
    let handle_motor_l = thread::spawn(move || {
        node_server_action_motor_l.talk();
    });
    info!("Motor L - Process started");
    let handle_motor_r = thread::spawn(move || {
        node_server_action_motor_r.talk();
    });
    info!("Motor R - Process started");
    let handle_led = thread::spawn(move || {
        node_server_action_led.talk();
    });
    info!("Status - Process started");
    std::thread::sleep(std::time::Duration::from_millis(50));
    let handle_ts = thread::spawn(move || {
        node_test.talk();
    });
    info!("Test - Process started");

    handle_led.join().unwrap();
    handle_motor_l.join().unwrap();
    handle_motor_r.join().unwrap();
    handle_st.join().unwrap();
    handle_ts.join().unwrap();
}

fn get_conns(names: Vec<&str>) -> HashMap<&str, &str> {
    // build connection list in a hashmap
    let mut all_conns: HashMap<&str, &str> = HashMap::new();
    all_conns.insert("led_action_server", "8101");
    all_conns.insert("motor_l_action_server", "8102");
    all_conns.insert("motor_r_action_server", "8103");
    all_conns.insert("status", "8201");
    all_conns.insert("test", "9000");
    // return only the ones needed
    let result: HashMap<&str, &str> = all_conns
        .into_iter()
        .filter(|(k, _v)| names.contains(&k))
        .collect();
    return result;
}
