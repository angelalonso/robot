# Architecture

## src/brain_boot.cpp

I honestly am not sure what this is for

## scripts/action_clients.py

Manages ACTION CLIENTS for:
- `MotorLeft`
- `MotorRight`
- `ServoLaser`
, each of them containing a send_goal function.

For the SERVOLASER it also has a goal_response_callback, a get_result_callback and a feedback_callback functions.

TODO: What are these three functions for?


## scripts/service_clients.py

Manages an ACTION CLIENT for `getstatuskey`, using:
- send_getstatuskey_req - Prepares request values for send_getstatuskey
- send_getstatuskey - Makes a service call to getstatuskey

TODO: why not add the setstatus one too?

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
- A client to `setstatus`
  - send_setstatus_req - Prepares request values for send_setstatus
  - send_setstatus - Makes a service call to setstatus
- The `getstatuskey` Service Client, loaded from scripts/service_clients.py
- An action client to Motor Left, loaded from scripts/action_clients.py
- An action client to Motor Right, loaded from scripts/action_clients.py

TODO: goals vs. goalsets vs. actionsets/actset

TODO: is the class Goal needed?


## scripts/node_api.py

It publishes an API server at localhost:5000 and connects each of the following to a set of actions:
- /do/stop - Stops both motors
- /do/fwd - Moves both motors forwards
- /do/bwd - Moves both motors backwards
- /do/right - Moves one motor back and the other forth, to rotate the robot to the right
- /do/left - Moves one motor back and the other forth, to rotate the robot to the right
- /do/scan - Moves the laser servo to a set of positions, and captures what was measured at that point by the laser

, to do that, it uses:
- An action client to Motor Left, loaded from scripts/action_clients.py
- An action client to Motor Right, loaded from scripts/action_clients.py
- An action client to Servo Laser, loaded from scripts/action_clients.py
To get the laser value, it uses getstatuskey Service Client, loaded from scripts/service_clients.py


## scripts/node_arduino.py

Defines the SerialLink class, which is mainly listening to anything that comes from the Arduino, and then sends messages to setstatus.
The idea is that Arduino grabs values from the sensors and pulls them to our robot. That is where this code intervenes and uses setstatus to store those values.

The SerialLink has the following components:
- A client to setstatus
  - send_setstatus_req - Prepares request values for send_setstatus
  - sanitize - Makes it possible to send several key-value on one message
  - send_from_sanitized - Makes a service call to setstatus
  - sync_and_read - This is the loop that listens for any messages coming from the Arduino

This code also accepts a MOCK mode, where values are read from a file instead

TODO: investigate why we are using self.publisher_


## scripts/node_callibrate.py

WIP that should be used to callibrate position and actions on the robot.

For instance:
Measure the position change when both motors run forward for 1 sec.

These callibrations should make it easier to then figure out position from how the motors have moved.


## scripts/node_motor_left.py
## scripts/node_motor_right.py

These two define `motorright_action_server` Action Servers and use GPIO to interact with the actual motors

The two commands they accept are `Forward` and `Backward` (Stop can also be used, but actually anything other than the previous two will stop the motor)


## scripts/node_servo_laser.py
## scripts/node_status.py
## scripts/service_status.py

## scripts/publisher_status.py

This one is trying to re-do service_status without using a service
