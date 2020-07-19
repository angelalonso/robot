use std::io;
use logwatch::Watcher;
//-- Choose an implementation --
use logwatch::PollWatcher as LogWatcher;
//use logwatch::TailWatcher as LogWatcher;

pub struct Message {
    pub incoming: bool,
    pub message: String,
}

pub struct Messages {
    pub received: Vec<Message>,
    pub transmitted: Vec<Message>,
}

impl Message {
    pub fn new(inc: bool, msg: String) -> Self {
        Self {
            incoming: inc,
            message: msg,
        }
    }
}

impl Messages {
    pub fn new() -> Self {
        let rx = Vec::new();
        let tx= Vec::new();
        Self {
            received: rx,
            transmitted: tx,
        }
    }
    pub fn add(&mut self, incoming: bool, message: String) {
        if incoming {
            self.received.push(Message {incoming, message})
        } else {
            self.transmitted.push(Message {incoming, message})
        }
    }
    // TODO: result can be an error too, we just send empty though
    pub fn get_list(&mut self, vec_type: &str) -> Option<Vec<String>> {
        match vec_type {
            "received" => {
                let mut recvvec = Vec::new();
                for entry in &mut self.received {
                    recvvec.push(entry.message.to_string());
                }
                Some(recvvec)
            },
            "transmitted" => {
                let mut trnsvec = Vec::new();
                for entry in &mut self.transmitted {
                    trnsvec.push(entry.message.to_string());
                }
                Some(trnsvec)
            },
            &_ => {
                None
            },
        }
    }
    // v1: read from a file
    // v2: read logs-wise
    // v2: wait, write somewhere else, read logs-wise
    pub fn factory<'a>(&mut self, count: &'a mut usize) -> Box<dyn FnMut(String) + 'a> {
        let callback = move |line: String| {
            *count += 1;
            println!("{}", line);
        };
        Box::new(callback)
    }
    pub fn read_the_buffer_on_test(&mut self) -> Result<(), io::Error> {
        let fpath = "testfile";
        let mut count = 0;
        let mut watcher = LogWatcher::new(&fpath, 2000); // polling period = 2000 milliseconds
        let callback = self.factory(&mut count);
        watcher.register(callback);
        watcher.watch();
        Ok(())
    }
    pub fn read_the_buffer() -> String {
        String::from("Test")
    }
}
