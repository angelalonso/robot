# First full example

## Plan your actions

For this first example we are using:
- a distance sensor
- an LED

Our goal is that the LED shines when we are closer than 20 cms from the sensor

## Connect all the required stuff

![Connection diagram](https://github.com/angelalonso/robot/tree/master/img/diagram.v2.1.png)  

## Define your rules
- Create a file called, for instance `rulesets/start_test1.yaml`
- Copy the following and modify it to your liking:
```
--- 
- id: "test_on"                    # each rule starts with an id. Try to avoid repeating ids
  condition:                       # condition block defines what triggers the rule
  - time: "0"                      # time here defines for how long the condition must be true before we apply it
    input_objs: "distance<20"      # this defines the conditions that must be true to trigger the rule
  actionsloop: false               # once the rule is triggered, should we apply actions in a loop?
  actions:                         # actions is a list of objects to set a value to
  - object: "led_y"                # object is the name of the object as defined in your setup.yaml
    value: "1"                     # value is the...value we will set the object to
    time: "0.1"                    # time is the amount of time the value stays (only applicable if there is another action for the same object later)
- id: "test_off"
  condition:
  - time: "0.2" 
    input_objs: "distance>=20,led_y=1"  # in this example we have several conditions. All must be true.
  actionsloop: false
  actions:                              # in this example we have several actions for the same object.
  - object: "led_y"                     #   , they will be applied in order
    value: "0"                          #   , and time for each action defines how long to wait until the next action can be applied
    time: "0.1"
  - object: "led_y"
    value: "1"
    time: "0.2"
  - object: "led_y"
    value: "1"
    time: "0.1"
```

# Challenges

