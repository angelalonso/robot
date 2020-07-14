pub struct CommRx {
    pub message: String,
}

pub struct CommTx {
    pub message: String,
}

pub struct Comms {
    pub received: Vec<CommRx>,
    pub transmitted: Vec<CommTx>,
}
