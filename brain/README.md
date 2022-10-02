# Architecture

## Schematics

- "Logic" Nodes
 - scripts/main_brain.py
 - scripts/publisher_status.py
 - scripts/action_clients.py

- "Hardware" Nodes
 - scripts/lib_fake_rpi.py
 - scripts/node_arduino.py
 - scripts/node_motor_left.py
 - scripts/node_motor_right.py
 - scripts/node_servo_laser.py

- "Frontend" Nodes
 - scripts/node_api.py

## Architectural decissions
- When a node fails, the execution continues unless we need the user to correct something from the get-go
  - E.g.: On callibration mode, we need the actionset file to exist, and the user should do that before starting
- Status management and finding actions that match the current statuses are done at the robotlogic lib written in Rust
