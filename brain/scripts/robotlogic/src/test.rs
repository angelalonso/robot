#![cfg(test)]

use super::*;

#[test]
fn integration_tests() {
    //let mut integration = botlogic::Logic::new("integration_actionset.yaml");
    let mut integration = Robotlogic::new("integration_actionset.yaml");
    // set state 
    integration.set_state("distance".to_string(), "11".to_string());
    // get action 
    assert_eq!(integration.get_action().unwrap(), "move");
    // set state 
    integration.set_state("time".to_string(), "0.0".to_string());
    integration.set_state("distance".to_string(), "9.0".to_string());
    // get action 
    assert_eq!(integration.get_action().unwrap(), "stop");
    // set state 
    integration.set_state("time".to_string(), "1.0".to_string());
    integration.set_state("distance".to_string(), "9.0".to_string());
    // get action 
    assert_eq!(integration.get_action().unwrap(), "stop");
    integration.set_state("time".to_string(), "0.0".to_string());
    integration.set_state("distance".to_string(), "11".to_string());
    // get action expecting an error
    let expected_err = Err(BrainDeadError::ActionNotFoundError);
    assert_eq!(integration.get_action(), expected_err);
}

/*
#[test]
fn state_basics() {
    let mut test = State::new();

    test.set("test", "check");
    test.set("mode", "mapping");

    assert!(test.get("testing").is_err());
    assert_eq!("check", test.get("test").unwrap());
}

#[test]
fn actionsfile_basics() {
    let mut test = Logic::new("actionset.yaml");
    test.set_state("test", "check");

    assert_eq!("check", test.get_state("test").unwrap());
}

#[test]
fn read_actionsfile() {
    let mut test = Logic::new("actionset.yaml");

    test.set_state("time", "1.0");

    assert_eq!("move", test.get_action().unwrap());

    test.set_state("time", "2.0");
    assert_eq!("scan", test.get_action().unwrap());

    test.set_state("time", "3.0");
    assert!(test.get_action().is_err());
}

#[test]
fn check_testcomparison() {
    let mut test = Logic::new("actionset.yaml");

    test.set_state("time", "1.0");
    assert_eq!(true, test.test_comparison("time!=1.01").unwrap());
    assert_eq!(true, test.test_comparison("time==1.0").unwrap());
    assert_eq!(true, test.test_comparison("time>=1.0").unwrap());
    assert_eq!(true, test.test_comparison("time>=0.9").unwrap());
    assert_eq!(true, test.test_comparison("time>0.9").unwrap());
    assert_eq!(true, test.test_comparison("time<=1.0").unwrap());
    assert_eq!(true, test.test_comparison("time<=1.1").unwrap());
    assert_eq!(true, test.test_comparison("time<1.01").unwrap());
}

#[test]
fn check_testconditions() {
    let mut test = Logic::new("actionset.yaml");
    test.set_state("time", "1");
    test.set_state("distance", "10");
    assert_eq!(false, test.test_conditions("time!=2&distance=10"));
}
*/
