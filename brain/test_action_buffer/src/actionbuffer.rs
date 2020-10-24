use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Action {
    pub action_type: String,
    pub value: String,
    pub time: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ActionBuffer {
    buffer: Vec<Action>,
    timer: f64,
}

impl ActionBuffer {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Self {
            buffer: [].to_vec(),
            timer: 0.0,
        })
    }

    pub fn add(&mut self, action: Action) {
        self.buffer.push(action);
    }

    pub fn print_all(&mut self) {
        for a in &self.buffer {
            println!("{:#x?}", a)
        };
    }
    pub fn do_all_lifo(&mut self) {
        while self.buffer.len() > 0 {
            let b = self.buffer.pop();
            println!("{:#x?}", b)
        };
    }
    pub fn do_next(&mut self, timestamp: f64) -> Result<Option<Action>, String>{
        if timestamp >= self.timer {
            if self.buffer.len() == 0 {
                self.timer = 0.0;
                Err("No more actions to take".to_string())
            } else {
                let a = &self.buffer.clone()[0];
                self.buffer.retain(|x| *x != *a);
                self.timer = a.time;
                Ok(Some(a.clone()))
            }
        } else {
            Ok(None)
        }
    }
    pub fn do_all(&mut self, start_time: f64) -> String {
        let mut latest_change = start_time;
        'outer: loop {
        //loop {
            let ct = SystemTime::now();
            let current_time = match ct.duration_since(UNIX_EPOCH) {
                Ok(time) => time.as_millis(),
                Err(_e) => 0,
            };
            let timestamp: f64 = (current_time as f64 - latest_change as f64) as f64 / 1000 as f64;
            match self.do_next(timestamp) {
                Ok(a) => match a {
                    Some(action) => {
                        println!("{:?} - {:?}", timestamp, action);
                        latest_change = current_time as f64;
                    },
                    None => (),
                },
                Err(e) => {
                    latest_change = current_time as f64;
                    break 'outer;
                },
            };
        }
        self.timer.to_string()
    }
}
