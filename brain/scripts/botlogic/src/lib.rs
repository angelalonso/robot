use std::collections::HashMap;

pub struct State<'a> {
    data: HashMap<&'a str, &'a str>
}


impl<'a> State<'a> {
    #[allow(dead_code)]
    fn new() -> State<'a> {
        State {data: HashMap::new()}
    }

    #[allow(dead_code)]
    fn set(&mut self, key: &'a str, val: &'a str) {
        self.data.insert(&key, &val);
    }

    #[allow(dead_code)]
    fn get(&mut self, key: &'a str) -> &str {
        return self.data[&key]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_basics() {
        let mut test = State::new();

        test.set("test", "check");
        test.set("mode", "mapping");

        assert_eq!("check", test.get("test"));
    }

    #[test]
    fn actionsfile_basics() {
        let _test = State::new();
    }
}
