use brain::brain::Brain;
use std::process;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn brain_init() -> Brain {
    let main_brain = Brain::new("Main Brain".to_string(), "dryrun".to_string(), "setup.yaml".to_string()).unwrap_or_else(|err| {
        eprintln!("Problem Initializing Main Brain: {}", err);
        process::exit(1);
    });
    return main_brain
}

fn brain_run(mut main_brain: Brain) {
    let (s, _r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
    let handle = thread::spawn(move || {
        let _actions = main_brain.run(Some(2.0), 100, s);
    });
    handle.join().unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
    let br = brain_init();
    c.sample_size(5);
    c.bench_function("action 20", |b| b.iter(|| brain_run(br.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
