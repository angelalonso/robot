# API reference
- POST 
  - /do
    Triggers an action. Available actions:
    - /do/stop
      Stop any previous movement.
    - /do/fwd
      Move Forward.
    - /do/bwd
      Move Backwards.
    - /do/right
      Rotate clockwise.
    - /do/left
      Rotate counter-clockwise.
    - /do/scan
      Rotates the scanner servo step by step and measures the distance to the next object.

  - /get
    Gets the status of a given parameter. Available parameters: 
    - /get/mode
      Gets the current mode the robot is running on.
