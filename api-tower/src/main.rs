#[macro_use]
extern crate tower_web;

use tower_web::ServiceBuilder;
use std::future;

#[derive(Clone, Debug)]
struct HelloWorld;

impl_web! {
    impl HelloWorld {
        /// @get("/")
        fn hello_world(&self) -> Result<String, ()> {
            Ok("Hello world".to_string())
        }

        /// @get("/moves")
        fn get_moves(&self) -> Result<String, ()> {
            Ok("MOVES".to_string())
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        .run(&addr)
        .unwrap();
}
