pub struct Action {
    pub name: String,
}

pub struct Actions {
    pub pending: Vec<Action>,
    pub done: Vec<Action>,
}
