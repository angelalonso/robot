#[cfg(test)]
mod config_tests {
    use crate::config::Config;

    #[test]
    fn get_actions() {
        let mut test = Config::new("testfiles/cfg.yaml");
        let getting_actions = test.get_actions("ping\r\n");
        assert!(getting_actions.is_ok(), "getting actions for existing trigger did not go well");
    } 

    #[test]
    fn get_actions_noactions() {
        let mut test = Config::new("testfiles/cfg.yaml");
        let getting_actions = test.get_actions("Result->Ping\n");
        assert!(getting_actions.is_err(), "getting actions for existing trigger did not go well");
    } 
}

