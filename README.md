# Robot v2.0.0

Resources to build a robot.

DISCLAIMER: I don't guarantee that your build will work, and I cannot guarantee that your robot will not catch fire.  
Use common sense before powering your build, soldering stuff or doing anything that might be remotely hazardous.

# WHY this guide?
- I know R:::OS and it looks much better than this, but sometimes all you need is reinvent the wheel to make it the exact size you need.
- I have very specific Hardware, most notably an <old> Raspberry pi B. Most of the docs online refer to the newer boards.  
- I want to share for other people to possibly improve on.

# Status

Second version worked and was configurable. Then my Arduino died on me and I had to go back to the drawing board to investigate why.
Basically any actions involving two wheels, four leds and a distance sensor should have been programmable.
There are some rough edges and hopefully all of them will be covered before version 3 arrives.
All in all, I'll have to use the chance to really understand my HW.

# Changes on v2
- TODO: there is a guide that goes from the bottom all the way up to deploying your first yaml ruleset to the robot.
  - TODO: this is basically a work on documenting electronics and solving potential issues there.
- We now just use one single setup file which defines inputs and outputs, first program to load in arduino and first ruleset to load.
- there is a mode called reset that should switch off all LEDs and motors. 
- there is a mode called test that mocks input from arduino and outputs to LEDs, motors...
- there is a mode called record that records what arduino gets as input on a logfile that can be used on tests

## What it can do
## Version 2

Version 2 includes a Step by Step Documentation that should guide you through the whole process

- [Shopping list before you start](https://github.com/angelalonso/robot/tree/master/docs/000_ShoppingList.md)
- [Notes on the setup](https://github.com/angelalonso/robot/tree/master/docs/001_Setup.md)
- [Installing your Raspberry](https://github.com/angelalonso/robot/tree/master/docs/002_Raspberry.md)
- [Installing your Arduino](https://github.com/angelalonso/robot/tree/master/docs/003_Arduino.md)
- [Building a basic chassis](https://github.com/angelalonso/robot/tree/master/docs/004_Chassis.md)
- [First example on basic connections](https://github.com/angelalonso/robot/tree/master/docs/005_first_example.md)

### Configuring
In order to configure, you'll need to create rulesets.

If you want to develop your ruleset and test it while at it, have a look at the [testing process](./brain/TESTING_HOWTO.md).

-- v1 --
## What it can do
Follow a list of rules regarding movements. Those rules can be based off what we get from the sensors. Basically it can do both of the following by choosing a different set of move_cfg.yaml rules:
- Time-based movements. One second to the front, two seconds turn to the left, ten seconds to the back...
- Objects avoiding robot. Move forward until an object is closer than 10 cms, then turn until there's nothing closer than 10 cms, then move forwards again.
- A mix of both. Move forward until an object is closer than 10 cms, rotate clockwise for a second, if still there's somthing closer than 10 cms, rotate counter-clockwise until there's nothing close than 10 cms, then move forwards again.

## Version 1
### Hardware
#### How to put it together
So far I don't have much more than a testing platform built with two small-ish boxes glued together and put on top of the Robot car kit chassis.  

![front_closed](https://github.com/angelalonso/robot/tree/master/img/chassis_front_closed.jpg) ![bottom](https://github.com/angelalonso/robot/tree/master/img/chassis_bottom.jpg)  
![open_bottom](https://github.com/angelalonso/robot/tree/master/img/chassis_front_open_bottom.jpg)
![open_top](https://github.com/angelalonso/robot/tree/master/img/chassis_front_open_top.jpg)    
### Connections
![Diagram as of September 2020](https://github.com/angelalonso/robot/tree/master/img/diagram.202010.png)

### Software
- Prepare the programs you want to run on the arudino. Check [ARDUINO's README](ARDUINO.md) for details on how the arduino programs should work, or have a look at the ones under ./arduino.
- Install everything required on the Raspberry pi. Check [our Raspberry Install process](RASPBERRY.md) for a HOW-TO.
- By the end of that process, and after you have everything connected, just plugin the battery in and everything should just work. Hopefully

### Challenges for v2
- It doesn't do much on its own. We should focus on skills.  
  - Those skills will need a means of callibration. Eg.: try to move straight, was it ok? save it, was it not? modify a bit, try again, ask for input.
  - That means we need some way to I/O physically with the robot. E.g: an LED and a button
- We need to add more sensors and find a way to make it flexible.
- Installation is not straghtforward
- Chassis is not reproducible
- Weight should be lowered, anything under 1 Kg is fine I guess  
- I have smaller powerbanks but they dont work well with the L298N motor controller  
- Further software challenges, check bottom of [BRAIN'S own README](https://github.com/angelalonso/robot/blob/master/brain/README.md)

 
