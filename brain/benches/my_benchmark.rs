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

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
    let mut br = brain_init();
    let secs_to_run = Some(1.0);
    let precission = 100;
    let start_timestamp = br.get_current_time();
    let mut ct: f64 = 0.0;
    // communication with arduino
    let (s, r): (Sender<String>, Receiver<String>) = std::sync::mpsc::channel();
    let msgs = s.clone();
    let mut arduino_clone = br.arduino.clone();
    let brain_clone = br.clone();
    thread::spawn(move || {
        if brain_clone.mode != "dryrun" {
            //TODO: find a way to reproduce this error, then add BrainDeadError to this function
            arduino_clone.read_channel(msgs).unwrap();
        } else {
            arduino_clone.read_channel_mock(msgs, brain_clone.setup_file.clone()).unwrap();
        };
    });
    match r.try_recv() {
        Ok(m) => br.use_arduino_msg(ct, m),
        Err(_) => (),
    };
    loop {
        let mut new_metrics: Vec<String> = [].to_vec();
        let mut new_actions: Vec<String> = [].to_vec();
        // update current timestamp
        let ct_raw = br.get_time_since(start_timestamp);
        let new_ct = (ct_raw * precission as f64).floor() / precission as f64;
        if new_ct > ct { 
            ct = new_ct;
            let _msg = match r.try_recv() {
                Ok(m) => br.use_arduino_msg(ct, m),
                Err(_) => (),
            };
            br.show_metrics();
            br.show_actionbuffers();
            // get actions
            // TODO: this is taking a while. Troubleshoot to see which one exactly and to try
            // and speed this up
            c.sample_size(50);
            c.bench_function("run_bench", |b| b.iter(|| 
                match br.get_actions_from_rules(ct){
                    Ok(actions) => {
                        // do first action from rules, add the rest to the actionbuffersets
                        let (these_metrics, these_actions) = br.do_actions_from_rules(actions.clone(), ct).unwrap();
                        for m_raw in these_metrics {
                            new_metrics.push(m_raw);
                        }
                        for c_raw in these_actions {
                            new_actions.push(c_raw);
                        }
                    },
                    Err(_e) => (),
                }
            ));
            // do action(s) from the actionbuffersets that match the ct
            let (these_metrics, these_actions) = br.do_actions_from_actionbuffersets(ct).unwrap();
            for m_raw in these_metrics {
                new_metrics.push(m_raw);
            }
            for c_raw in these_actions {
                if c_raw != "" {
                    new_actions.push(c_raw);
                }
            }
            // add metrics to metricsets
            for m_raw in new_metrics.clone() {
                let m = m_raw.split("|").collect::<Vec<_>>();
                if m.len() > 1 {
                    br.add_metric(ct, m[0].to_string(), m[1].to_string());
                }
            }
        };
        // break mechanism
        match secs_to_run {
            Some(s) => {
                if ct >= s {
                    println!("Finished execution");
                    break
                }
            },
            None => (),
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
