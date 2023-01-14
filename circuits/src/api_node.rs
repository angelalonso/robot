// thanks to https://github.com/tmsdev82/rust-warp-rest-api-tutorial
use crate::comms::*;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{http::StatusCode, reply, Filter, Rejection, Reply};

type ItemNode = Arc<Mutex<String>>;
type ItemComms<'a> = Arc<Mutex<UDPComms<'a>>>;
type Result<T> = std::result::Result<T, Rejection>;

pub struct ApiNode<'a> {
    port_in: &'a str,
    conns: HashMap<&'a str, &'a str>,
}

impl<'a> ApiNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        let node = match get_port(name, conns.clone()) {
            Ok(c) => ApiNode { port_in: c, conns },
            Err(_) => {
                panic!(
                    "couldn't find a port to itself: {} (HINT: check name at main.rs)",
                    name
                );
            }
        };
        node
    }
    pub async fn talk(&mut self) {
        let mut nodes: HashMap<String, String> = HashMap::new();
        nodes.insert(
            "led_action_server".to_owned(),
            get_port("led_action_server", self.conns.clone())
                .unwrap()
                .to_owned(),
        );
        nodes.insert(
            "motor_l_action_server".to_owned(),
            get_port("motor_l_action_server", self.conns.clone())
                .unwrap()
                .to_owned(),
        );
        nodes.insert(
            "motor_r_action_server".to_owned(),
            get_port("motor_r_action_server", self.conns.clone())
                .unwrap()
                .to_owned(),
        );
        let comms = UDPComms::new(self.port_in.to_owned());
        run(nodes, comms).await;
    }
}

async fn run(nodes: HashMap<String, String>, comms_orig: UDPComms<'static>) {
    // TODO: use the same logging format, pass the log level to both
    // TODO: use proper API actions, Get, Post...only when it makes sense
    let led_node: ItemNode = Arc::new(Mutex::new(
        nodes.get("led_action_server").unwrap().to_string(),
    ));
    let motor_l_node: ItemNode = Arc::new(Mutex::new(
        nodes.get("motor_l_action_server").unwrap().to_string(),
    ));
    let motor_r_node: ItemNode = Arc::new(Mutex::new(
        nodes.get("motor_r_action_server").unwrap().to_string(),
    ));
    let comms: ItemComms = Arc::new(Mutex::new(comms_orig));
    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let get_empty_route = warp::path("get")
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(get_empty);

    let get_route = warp::path!("get" / "mode" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(get_mode);

    let do_scan_route = warp::path!("do" / "scan" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_scan);

    let do_stop_route = warp::path!("do" / "stop" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_node(motor_l_node.clone()))
        .and(with_node(motor_r_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_stop);

    let do_fwd_route = warp::path!("do" / "fwd" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_node(motor_l_node.clone()))
        .and(with_node(motor_r_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_fwd);

    let do_bwd_route = warp::path!("do" / "bwd" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_node(motor_l_node.clone()))
        .and(with_node(motor_r_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_bwd);

    let do_left_route = warp::path!("do" / "left" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_node(motor_l_node.clone()))
        .and(with_node(motor_r_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_left);

    let do_right_route = warp::path!("do" / "right" / ..)
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_node(motor_l_node.clone()))
        .and(with_node(motor_r_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(do_right);

    let routes = root
        .or(get_route)
        .or(get_empty_route)
        .or(do_scan_route)
        .or(do_stop_route)
        .or(do_fwd_route)
        .or(do_bwd_route)
        .or(do_left_route)
        .or(do_right_route)
        .with(warp::cors().allow_any_origin());

    // TODO: make this an env var
    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}

fn with_node(
    node_name: ItemNode,
) -> impl Filter<Extract = (ItemNode,), Error = Infallible> + Clone {
    warp::any().map(move || node_name.clone())
}

fn with_comms(comms: ItemComms) -> impl Filter<Extract = (ItemComms,), Error = Infallible> + Clone {
    warp::any().map(move || comms.clone())
}

pub async fn get_empty(_led_n: ItemNode, _comms_orig: ItemComms<'_>) -> Result<impl Reply> {
    let result = "";
    println!("test {}", result);
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn get_mode(led_n: ItemNode, comms_orig: ItemComms<'_>) -> Result<impl Reply> {
    let led = led_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to(&"SET:led:on".as_bytes().to_vec(), &led);
    let result = format!(" get mode");
    println!("{}", result);
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_scan(_led_n: ItemNode, _comms_orig: ItemComms<'_>) -> Result<impl Reply> {
    let result = "";
    println!("do scan");
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_stop(
    _led_n: ItemNode,
    motor_l_n: ItemNode,
    motor_r_n: ItemNode,
    comms_orig: ItemComms<'_>,
) -> Result<impl Reply> {
    let motor_l = motor_l_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to(&"SET:stp".as_bytes().to_vec(), &motor_l);
    let motor_r = motor_r_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to(&"SET:stp".as_bytes().to_vec(), &motor_r);
    let result = "";
    println!("do stop");
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_fwd(
    _led_n: ItemNode,
    motor_l_n: ItemNode,
    motor_r_n: ItemNode,
    comms_orig: ItemComms<'_>,
) -> Result<impl Reply> {
    let motor_l = motor_l_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to(&"SET:fwd".as_bytes().to_vec(), &motor_l);
    let motor_r = motor_r_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to(&"SET:fwd".as_bytes().to_vec(), &motor_r);
    let result = "";
    println!("do forward");
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_bwd(
    _led_n: ItemNode,
    motor_l_n: ItemNode,
    motor_r_n: ItemNode,
    comms_orig: ItemComms<'_>,
) -> Result<impl Reply> {
    let motor_l = motor_l_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to(&"SET:bwd".as_bytes().to_vec(), &motor_l);
    let motor_r = motor_r_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to(&"SET:bwd".as_bytes().to_vec(), &motor_r);
    let result = "";
    println!("do backwards");
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_left(
    _led_n: ItemNode,
    motor_l_n: ItemNode,
    motor_r_n: ItemNode,
    comms_orig: ItemComms<'_>,
) -> Result<impl Reply> {
    let motor_l = motor_l_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to(&"SET:bwd".as_bytes().to_vec(), &motor_l);
    let motor_r = motor_r_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to(&"SET:fwd".as_bytes().to_vec(), &motor_r);
    let result = "";
    println!("do left");
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn do_right(
    _led_n: ItemNode,
    motor_l_n: ItemNode,
    motor_r_n: ItemNode,
    comms_orig: ItemComms<'_>,
) -> Result<impl Reply> {
    let motor_l = motor_l_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to(&"SET:fwd".as_bytes().to_vec(), &motor_l);
    let motor_r = motor_r_n.lock().await;
    comms_orig
        .lock()
        .await
        .send_to(&"SET:bwd".as_bytes().to_vec(), &motor_r);
    let result = "";
    println!("do right");
    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}
