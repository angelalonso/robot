# Architecture

## src/brain_boot.cpp

I honestly am not sure what this is for

## scripts/action_clients.py

Manages ACTION CLIENTS for:
- MotorLeft
- MotorRight
- ServoLaser
, each of them containing a send_goal function.

For the SERVOLASER it also has a goal_response_callback, a get_result_callback and a feedback_callback functions.

TODO: What are these three functions for?


## scripts/lib_fake_rpi.py

It contains a FAKE_RPI, a PWM_OBJECT and a FAKE_PIGPIO objects that give us dummy responses and we can use when we run our robot on a machine that is not a Raspberry Pi (e.g.: in our laptops for testing)


## scripts/main_brain.py

It manages timed goal sets.

It does so with the following components:
- A list of Goal Sets
  - load_goalsets - Loads an initial list from goalsets/main.yaml
  - add_goals - Adds goals from a goalset to the list
  - trigger_goalsets - Checks the time among other conditions and calls apply_goal on the goals that should run
  - apply_goal - Triggers the goal to the related object (e.g.: Motor Left)
  - clear_goals - removes all goals on the list
  - get_goalset - Returns a goalset with a given name form the list of goalsets
  - set_goalset_active - Sets a given goalset to active, and all others to inactive
  - set_goalset_done - Sets a given goalset to started: False
- A client to setstatus
  - send_getstatuskey_req - Prepares request values for send_getstatuskey
  - send_getstatuskey - Makes a service call to getstatuskey
- A client to getstatuskey
  - send_setstatus_req - Prepares request values for send_setstatus
  - send_setstatus - Makes a service call to setstatus
- An action client to Motor Left
- An action client to Motor Right

TODO: goals vs. goalsets vs. actionsets/actset

TODO: is the class Goal needed?


## scripts/node_api.py
## scripts/node_arduino.py
## scripts/node_callibrate.py
## scripts/node_motor_left.py
## scripts/node_motor_right.py
## scripts/node_servo_laser.py
## scripts/node_status.py
## scripts/service_clients.py
## scripts/service_status.py

