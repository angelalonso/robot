use circuits::api_node::*;
use circuits::arduino_node::*;
use circuits::get_conns;
use circuits::led_action_server_node::*;
use circuits::motor_l_action_server_node::*;
use circuits::motor_r_action_server_node::*;
use circuits::status_node::*;
use circuits::test_node::*;

use chrono::Local;
use env_logger::Builder;
use load_dotenv::load_dotenv;
use log::info;
use std::io::Write;
use std::thread;

#[tokio::main]
async fn main() {
    load_dotenv!();
    let mut handles = vec![];

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

    let mut node_server_action_led =
        LedActionServerNode::new("led", get_conns(["led", "status", "test"].to_vec()));
    let mut node_server_action_motor_l =
        MotorLActionServerNode::new("motor_l", get_conns(["motor_l", "status", "test"].to_vec()));
    let mut node_server_action_motor_r =
        MotorRActionServerNode::new("motor_r", get_conns(["motor_r", "status", "test"].to_vec()));
    let mut node_api = ApiNode::new(
        "api",
        get_conns(["motor_l", "motor_r", "led", "api", "status", "test"].to_vec()),
    );
    let mut node_arduino =
        ArduinoNode::new("arduino", get_conns(["arduino", "status"].to_vec()), false);
    let mut node_status = StatusNode::new(
        "status",
        get_conns(["motor_l", "motor_r", "led", "status", "test"].to_vec()),
    );
    let mut node_test = TestNode::new(
        "test",
        get_conns(["motor_l", "motor_r", "led", "status", "test"].to_vec()),
    );

    std::thread::sleep(std::time::Duration::from_millis(50));
    let handle_st = thread::spawn(move || {
        node_status.talk();
    });
    handles.push(handle_st);
    info!("Status - Process started");
    node_arduino.connect();
    let handle_ar = thread::spawn(move || {
        node_arduino.talk();
    });
    handles.push(handle_ar);
    info!("Arduino - Process started");
    let handle_motor_l = thread::spawn(move || {
        node_server_action_motor_l.talk();
    });
    handles.push(handle_motor_l);
    info!("Motor L - Process started");
    let handle_motor_r = thread::spawn(move || {
        node_server_action_motor_r.talk();
    });
    handles.push(handle_motor_r);
    info!("Motor R - Process started");
    let handle_led = thread::spawn(move || {
        node_server_action_led.talk();
    });
    handles.push(handle_led);
    info!("Led - Process started");
    std::thread::sleep(std::time::Duration::from_millis(50));
    let handle_ts = thread::spawn(move || {
        node_test.talk();
    });
    handles.push(handle_ts);
    info!("Test - Process started");

    // This one goes at the end
    //let handle_api = thread::spawn(move || {
    node_api.talk().await;
    //});
    //handles2.push(handle_api);
    info!("API - Process started");
    for handle in handles {
        handle.join().unwrap();
    }
}
