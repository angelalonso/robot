use roctl::climenu::parser;
use roctl::roctl::proxy_action;

//#[macro_use]
extern crate log;

fn main() {
    env_logger::builder()
    .format_timestamp_millis()
    .init();

    proxy_action(parser());
}
