use super::*;

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
    test.set_state("time", "1.0");
    test.set_state("distance", "10.0");
    assert_eq!(true, test.test_condition("time!=1.01&&distance==10.0").unwrap());
    assert_eq!(false, test.test_condition("time!=1.0&&distance==11.0").unwrap());

    assert_eq!(true, test.test_condition("time!=1.0||distance==10.0").unwrap());
    assert_eq!(true, test.test_condition("time==1.0||distance==10.0").unwrap());
    assert_eq!(false, test.test_condition("time!=1.0||distance==11.0").unwrap());
    assert_eq!(true, test.test_condition("time!=1.0||distance==10.0").unwrap());
    // TODO:
    // test several ands, ors
    // test grouping on ()
}
