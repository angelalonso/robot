use brain::brain::Brain;
use std::process;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn actions_obstacle() {
    let mut main_brain = Brain::new("Main Brain".to_string(), "dryrun".to_string(), "setup.yaml".to_string()).unwrap_or_else(|err| {
        eprintln!("Problem Initializing Main Brain: {}", err);
        process::exit(1);
    });
    let (s, _r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
    let handle = thread::spawn(move || {
        let _actions = main_brain.run(Some(2.0), 100, s);
    });
    handle.join().unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
    c.sample_size(5);
    c.bench_function("action 20", |b| b.iter(|| actions_obstacle()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
