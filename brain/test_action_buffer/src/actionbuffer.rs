
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Action {
    pub action_type: String,
    pub value: String,
    pub time: f64,
}

pub struct ActionBuffer {
    buffer: Vec<Action>,
}

impl ActionBuffer {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Self {
            buffer: [].to_vec(),
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
    pub fn do_all(&mut self) {
        for a in self.buffer.clone() {
            self.buffer.retain(|x| *x != a);
            println!("{:#x?}", a)
        };
    }
    //TODO: do "do_all" based on time
}
