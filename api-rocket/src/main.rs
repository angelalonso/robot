#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::State;

#[get("/")]
fn all(channel: State<String>) -> String {
    channel.to_string()
}

fn main() {
    let channel = "TEST".to_string();
    rocket::ignite()
        .manage(channel)
        .mount("/",
               routes![all],
               ).launch();
}
