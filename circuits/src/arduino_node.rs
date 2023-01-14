pub struct ArduinoNode {
    connected: bool,
}

impl ArduinoNode {
    pub fn new() -> Self {
        let node = ArduinoNode { connected: false };
        node
    }

    pub fn connect(&mut self) {
        self.connected = true;
    }
}
