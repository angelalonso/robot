use std::fs::File;

extern crate serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigEntry {
    trigger: String,
    actions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    entries: Vec<ConfigEntry>,
}

impl Config {
    pub fn new(configfile: &str) -> Self {
        let filepointer = File::open(configfile).unwrap();

        let reactions: Vec<ConfigEntry> = serde_yaml::from_reader(filepointer).unwrap();
        Self {
            entries: reactions,
        }
    }

    pub fn print(&mut self) {
        println!("{:?}", self.entries);
    }

    pub fn get_actions(&mut self, configentry: &str) -> Option<Vec<String>> {
        let entry = self.entries.iter()
            .find(|&x| x.trigger == configentry.to_string());
        match entry {
            Some(x) => Some(x.actions.to_vec()),
            None => None,
        }
    }
}
