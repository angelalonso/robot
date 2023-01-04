use crate::comms::*;

use log::debug;
use std::collections::HashMap;

use hyper::body::Buf;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{header, Body, Method, Request, Response, StatusCode};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct Car {
    id: String,
    brand: String,
    model: String,
    year: u16,
}

const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

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
    //pub async fn talk(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
    pub async fn talk(&mut self) {
        let led_node = get_port("led_action_server", self.conns.clone()).unwrap();
        let comms = UDPComms::new(self.port_in.to_owned());
        match run().await {
            Ok(_) => println!("OK"),
            Err(_) => println!("Err"),
        };
    }
}
// ---------------------------------------------

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(api_handler))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn api_handler(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    let path = req.uri().path().to_owned();
    let path_segments = path.split("/").collect::<Vec<&str>>();
    let base_path = path_segments[1];

    match (req.method(), base_path) {
        // TODO: fill this up
        // TODO: change android app to use GET for these
        (&Method::GET, "get") => {
            if path_segments.len() <= 2 {
                let res = "Empty, TBD";
                println!("answering {}", res);
                return Ok(build_response(res.to_owned()));
            }

            let obj_id = path_segments[2];

            if obj_id.trim().is_empty() {
                let res = "Empty but with no stuff too, TBD";
                println!("answering {}", res);
                return Ok(build_response(res.to_owned()));
            } else {
                let res = "NOT Empty, TBD";
                println!("answering {}", res);
                return Ok(build_response(res.to_owned()));
            }
        }

        (&Method::POST, "get") => {
            if path_segments.len() <= 2 {
                let res = "Empty, TBD";
                println!("answering {}", res);
                return Ok(build_response(res.to_owned()));
            }

            let obj_id = path_segments[2];

            if obj_id.trim().is_empty() {
                let res = "Empty GET but with no stuff too, TBD";
                println!("answering {}", res);
                return Ok(build_response(res.to_owned()));
            } else {
                let res = "NOT Empty GET, TBD";
                println!("answering {}", res);
                return Ok(build_response(res.to_owned()));
            }
        }

        (&Method::POST, "do") => {
            if path_segments.len() <= 2 {
                let res = "Empty DO , TBD";
                println!("answering {}", res);
                return Ok(build_response(res.to_owned()));
            }

            let obj_id = path_segments[2].trim();

            if obj_id.is_empty() {
                let res = "Empty DO but with no stuff too, TBD";
                println!("answering {}", res);
                return Ok(build_response(res.to_owned()));
            } else {
                match obj_id {
                    // TODO: change this to stp
                    "stop" => {
                        let res = "DO STP, TBD";
                        println!("answering {}", res);
                        return Ok(build_response(res.to_owned()));
                    }
                    "fwd" => {
                        let res = "DO FWD, TBD";
                        println!("answering {}", res);
                        return Ok(build_response(res.to_owned()));
                    }
                    "bwd" => {
                        let res = "DO BWD, TBD";
                        println!("answering {}", res);
                        return Ok(build_response(res.to_owned()));
                    }
                    "right" => {
                        let res = "DO RIGHT, TBD";
                        println!("answering {}", res);
                        return Ok(build_response(res.to_owned()));
                    }
                    "left" => {
                        let res = "DO LEFT, TBD";
                        println!("answering {}", res);
                        return Ok(build_response(res.to_owned()));
                    }
                    "scan" => {
                        let res = "DO SCAN, TBD";
                        println!("answering {}", res);
                        return Ok(build_response(res.to_owned()));
                    }
                    _ => {
                        let res = "NOT Empty but wrong DO, TBD";
                        println!("answering {}", res);
                        return Ok(build_response(res.to_owned()));
                    }
                }
            }
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn build_response(text: String) -> Response<Body> {
    let res: [String; 1] = [text];

    match serde_json::to_string(&res) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    }
}
