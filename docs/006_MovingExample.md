# Next Example: mvoe the wheels based on obstacles

## Plan your actions

For this first example we are using:
- a distance sensor
- two wheels through an L298N

Our goal is that 
- If there is nothing clsoer than 20 cms, the wheels move forward.
- If there is something closer than 20 cms:
  - The wheels move backward for half a second.
  - After that half a second, one wheel moves forward, another one moves backwards for another second.
  - Obviously during all the process, we check the distance to any object and correct the phase we are in

## Connect all the required stuff

![Connection diagram](../img/diagram.test2.png)  

## Define your rules
- Create the ruleset and call it, for instance `rulesets/start_test2.yaml`
- Copy the following and modify it to your liking (see [previous example](./005_FirstExample.md) for references of what variable does):
```
--- 
```

### Optional: dry test your rules
If you want to dryrun your ruleset without installing to the robot, you can do the following:
- create a new setup.yaml, call it setup_test.yaml for instance.
  - set start_actionset_file to rulesets/start_test1.yaml 
- create a new mock input file. This file "injects" messages that would be sent by the Arduino in a live setup. 
  - If your setup file was named setup_test.yaml, this file MUST be called mock_test.yaml
  - An example of the content there would look like:
```
--- 
- id: "move_fwd"
  condition:
  - time: "0.05" 
    input_objs: "distance>20"
  actionsloop: false
  actions:
  - object: "motor_l"
    value: "80"
    time: "0.1"
  - object: "motor_r"
    value: "80"
    time: "0.1"
- id: "stop_n_turn"
  condition:
  - time: "0.05" 
    input_objs: "distance<=20,motor_l=80,motor_r=80"
  actionsloop: false
  actions:
  - object: "motor_l"
    value: "-80"
    time: "0.5"
  - object: "motor_r"
    value: "-80"
    time: "0.5"
  - object: "motor_l"
    value: "80"
    time: "1.0"
  - object: "motor_r"
    value: "-80"
    time: "1.0"
```
- Run on test mode
  - Since your goal is to check that it works well, we'll need at least the INFO verbosity output:
```
RUST_LOG=info cargo run test setup_test.yaml
```
, or, if you need an even more verbose level of verbose output you can use DEBUG or TRACE:
```
RUST_LOG=debug cargo run test setup_test.yaml
```


# Challenges

[PREV: Build a Chassis <--](004_Chassis.md)
