use rand::Rng;
use std::{thread, time};

async fn first() -> String {
    let rnd_sleep = rand::thread_rng().gen_range(500..1000);
    let sleep_millis = time::Duration::from_millis(rnd_sleep);
    thread::sleep(sleep_millis);
    println!("first");
    return "FIRST OUT!".to_owned();
}

async fn second() -> String {
    let rnd_sleep = rand::thread_rng().gen_range(500..1000);
    let sleep_millis = time::Duration::from_millis(rnd_sleep);
    thread::sleep(sleep_millis);
    println!("second");
    return "SECOND OUT!".to_owned();
}

#[tokio::main]
async fn main() {
    let mut handles = vec![];
    let handle_f = tokio::spawn(async move {
        let _f = first().await;
    });
    handles.push(handle_f);
    let handle_s = tokio::spawn(async move {
        let _s = second().await;
    });
    handles.push(handle_s);

    for handle in handles {
        handle.await.unwrap();
    }
}
