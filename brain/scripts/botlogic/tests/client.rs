// integration tests
// - Create from a yaml file
// - add values
// - wait and receive actions

use botlogic;

#[test]
fn integration_tests() {
    let mut integration = botlogic::Logic::new("integration_actionset.yaml");
    // set state 
    integration.set_state("distance", "20.0");
    // get action 
    assert_eq!(integration.get_action().unwrap(), "that action");
}
