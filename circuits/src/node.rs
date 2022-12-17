use rand::Rng;
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub struct Msgs {
    pub content: String,
}

#[derive(Debug)]
pub struct Node {
    id: String,
    pub name: String,
    msg_in: String,
    msg_prev: String,
}

impl Node {
    pub fn new<'a>(id: String, name: String) -> Node {
        Node {
            id: id,
            name: name,
            msg_in: "".to_string(),
            msg_prev: "".to_string(),
        }
    }

    pub async fn run(&mut self, mut comm: tokio::sync::MutexGuard<'_, String>) {
        let rnd_sleep = rand::thread_rng().gen_range(1000..1200);
        let sleep_millis = time::Duration::from_millis(rnd_sleep);
        thread::sleep(sleep_millis);
        *comm = self.name.clone();
        println!("------ {} - {:#?}", self.id, self.name);
        println!("------   msg prev->{}", self.msg_prev);
        self.msg_in = format!("{}", *comm);
        self.msg_prev = format!("{}", self.msg_in);
        println!("------   msg in  ->{}", self.msg_in);
        println!("------   msg prev2  ->{}", self.msg_prev);
    }
}
