
#[derive(Debug, Clone)]
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
    //TODO: add an order on the buffer, read them in that order (and empty the buffer)
    //TODO: do that read based on time
}
