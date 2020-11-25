# How to configure the Robot

## SETUP file
This file describes your erm, setup.   

Then name is unimportant because you'll pass it as a parameter to the brain program.  

You can find an example at setup.yaml on this directory.  

Modify to your liking with the following information:   
- start_actionset_file is the first rule/actions file to load to the robot program.
- start_arduinohex_file is the first program to load to the arduino.
- inputs is a list of sensors that give input to the robot.
  - below them, you just list them by the name you are using on your arduino programs.
- outputs is a list of things you modify from the program, like LEDs or the motors.  
  - we use a single string for name and gpio pins(NOT the pin number but the GPIO value of the pin), separated by __ (double underscores).
  - So far we only can add LEDs and Motors
    - LEDS are defined like led_y__gpio=21
    - MOTORS are defined like motor_l__gpio=27__gpio=17__gpio=22, where the third gpio is the enabler one.
*NOTE: THe above mentioned needs review, check setup.yaml for an example on how it works now
### Challenges
- We should look into using a similar setup file as R OS.

## RULESETS file(s)
Also known as the Actions file, it defines conditions (when needed) that trigger actions.

You can have as many as you need, and even call one from another (see "special objects" below) if you need to. 

I suggest you add them in the rulesets/ folder, where you'll already find some examples, but any other form of keeping those files should be fine.

Modify them or create one from scratch with the following information:
- Each file can store several rulesets
- id is the name you want to give it. Try to avoid repeating ids.  
- actionsloop defines whether the actions will be repeated or applied just once.
- condition is the... conditions that would trigger the actions. 
  - time is the amount of time in seconds that the conditions have to be there before the actions get applied. 
  - input_objs is the list of conditions, separated by a , (comma)
    - The work like a filter.
    - Example: "button=1,motor_l=0,motor_r=0"
    - An empty input_objs means the actions will be triggered right away.
- actions is the list of actions to take, like:
```
  - object: "led_y"
    value: "1"
    time: "0.2"
  - object: "led_y"
    value: "0"
    time: "0.4"
  - object: "led_r"
    value: "0"
    time: "0.3"
```
  - All actions for a given object get queued together, so the above example means led_y will be 1 for 0.2 secconds, THEN 0 for another 0.4 seconds, while led_r will be 0 for 0.3 starting from the beginning
  - There is a couple "special" objects that go in the same queue:
    - load, which loads a new actions file and looks like:
    ```
  - object: "load"
    value: "ruleset/start.yaml" <- this file MUST exist in the declared path
    time: "0.2"
    ```
    - wait, which just waits (helps control when the other "special" actions, like load, happen)
    ```
  - object: "wait"
    value: "" <- this value is not needed and ignored
    time: "5"
    ```
### Strategies

This section should give you a hint on how to make things work

#### Button changes a, then changes a differently a second time
Let's say you want a button to make motors go forward, then stop on a second click:
- Create an action file and add to actionsets
- Give both the same id
- Obviously define different outputs, but add a "load" (with a wait) to the actionfile itself at the bottom
- Then have different input_objs based on what each is doing (one makes motor=0 and has inputobj motor=70 and the other makes motor=70 and has inputobj motor=0)
- See an example at testfiles/button_cfg.yaml

### Challenges
- Actions with an input are run only once. E.g.: you push a button, action gets triggered, but from there on it doesn't react to it
