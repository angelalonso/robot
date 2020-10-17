# Brain

Program to manage the Robot.

The program manages: 
- Communications with Arduino
- Installing new .hex files into the Arduino
- Movement of the motors

## Configuration

Before a regular start, you need to configure two files:
- cfg.yaml
  - Defines actions taken after a trigger.
  - Triggers are received as a message from the arduino (see [arduino README](./ARDUINO.md))
  - Each entry has the following form:
```
- trigger: "wheel finished"
  actions:
    - install_../arduino/000_ready/000_ready.ino.hex
```
  - , where install\_ is the action to take. Available actions are:
    - install, to install a program to the Arduino, where everything after the \_ is the path to the hex file to install
    - move, to move the motors, where everything after the \_ defines the values we want for each enging (Left_Right, as in 60_60 or -40_70)
- move_cfg.yaml


## Edit the -> config file
- edit cfg.yaml
- For each action, add one of the following:
```
- trigger: "ping"
  actions:
    - install_../tests/001_test_pong/001_test_pong.ino.hex  <- here you just put install_<and the path to the next .ino.hex file you want to install to the arduino>
```


## Run tests
- run cp env.template .env
- modify .env to your needs 
- run ./test_on_raspberry.sh 


# Challenges
- Event driven but only up to one layer. We need more flexibility.
- 
