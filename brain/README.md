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
    - move, to move the motors
      - here we define the speed (-100 to 100) of each motor (Left_Right, as in 60_60 or -40_70) after the \_.
- move_cfg.yaml
  - Defines the next value for the motorsm based on the values of some metrics.
  - Those metrics are currently:
    - speed of each motor
    - distance received by the L298N
    - value of the tracking sensor 
    - time since last change on metrics
  - Each entry has the following form:
```
- input:
  - time: "1" 
    motor_l: "60"
    motor_r: "60"
    tracker: "*"
    distance: "<=_30"
  action:
    motor_l: -60
    motor_r: -60
```
  - where each of the inputs can take "\*" as a value if needs to be ignored, and then you can set each as follows if needed:
    - time - seconds (accepts one dec) since last metric change (useful when you need to say "if it has been moving forward for two seconds, stop")
      - It gets triggered when the time measured is equal or bigger than this
    - motor_l and motor_r  are the speed of each motor, from -100 to 100 with 0 being a full stop.
      - It triggered only when the values measured and the input are the exact same.
      - The motors I use only actuall move for values over 55 (and -55)
    - tracker has a true/false valuu and works like the previous ones.
    - distance has a format of "comparison_value", and the metric gets stored like that (instead of the actual value), meaning two metrics of 20 and 26 might both count as the same input for something like "<=\_30" 

## Run the code
You will start your robot from the laptop once it is up, running and connected to your wifi.  
I am assuming you know your robot's IP (if not, google).  
- Copy the env template to generate your own
```
cp env.template .env
```
- Edit and fill up .env to your needs.  
- Run the remote script
```
./install_and_run.sh
```
  - You might want to check and modify that script for things like changing the log level.
- Honestly, at this point you can always use cargo build and call the exec file at your Raspberry directly.

# Challenges
- We are not actually building any exec, and we should.
- We need a way for the robot to load Brain on bootup. This should include an automated update if necessary.  
- We need to document or automate finding out the Robot's IP.
- Event driven but only up to one layer. We need more flexibility.
- We dont have a real way to test on a laptop

