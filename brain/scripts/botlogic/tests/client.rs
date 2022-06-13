// integration tests
// - Create from a yaml file
// - add values
// - wait and receive actions

use botlogic;

#[test]
fn integration_tests() {
    let mut integration = botlogic::Logic::new("integration_actionset.yaml");
    // set state 
    integration.set_state("distance", "11");
    // get action 
    assert_eq!(integration.get_action().unwrap(), "move");
    // set state 
    integration.set_state("time", "0.0");
    integration.set_state("distance", "9.0");
    // get action 
    assert_eq!(integration.get_action().unwrap(), "stop");
    // set state 
    integration.set_state("time", "1.0");
    integration.set_state("distance", "9.0");
    // get action 
    assert_eq!(integration.get_action().unwrap(), "stop");
    integration.set_state("time", "0.0");
    integration.set_state("distance", "11");
    // get action expecting an error
    let expected_err = Err(botlogic::BrainDeadError::ActionNotFoundError);
    assert_eq!(integration.get_action(), expected_err);
}
