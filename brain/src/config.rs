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
    // ------------------------------------------------------ //
    pub fn print(&mut self) {
        println!("{:?}", self.entries);
    }
    // ------------------------------------------------------ //
    pub fn get_actions(&mut self, raw_configentry: &str) -> Option<Vec<String>> {
        let configentry = raw_configentry.to_lowercase();
        let entry = self.entries.iter()
            .find(|&x| x.trigger.to_lowercase() == configentry);
        match entry {
            Some(x) => Some(x.actions.to_vec()),
            None => None,
        }
    }
}
// ---------------------------------------------------------- //
#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn print() {
        let mut test = Config::new("testfiles/test.cfg.yaml");
        test.print();
    } 

    #[test]
    fn get_actions() {
        let mut test = Config::new("testfiles/test.cfg.yaml");
        test.get_actions("Result->Ping\n");
    } 
}
