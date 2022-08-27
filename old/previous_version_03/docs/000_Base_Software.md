# How to run this Software

I develop and test on a laptop before it runs on the Robot.

Let's assume you are doing everything on the Raspberry only, in that case the first you need to do is SSH into the Raspberry (see SSH configuration on [[Raspberry Pi setup doc]](docs/000_Base_Raspberry.md) ).

# Clone this repo
```
$ git clone https://github.com/angelalonso/robot
```

# Configure the robot
```
$ cd $HOME/robot/brain
$ cp env.template .env
```
, and modify .env accordingly (for instance, if you are using different GPIO Pins)


# Start the robot

```
$ cd $HOME/robot/brain
$ ./run.sh build
```
, where build will only be needed on the first run. From then on just run:
```
$ ./run.sh
```

# The Actionset

The file brain/goalsets/main.yaml contains a list of goalsets.

Whether a Goalset is run or not depends on two factors:
- a list of conditions under conditions_or
  - We store status on self.status['element_name'], the condition definition should use that
    - E.g.: you can have one condition like "self.status['distance'] >= 35"
  - As the name suggest, we just need one of them to be True
    - If you need to use ANDs, put them in one condition, Python-style
    - E.g.: "(self.status['distance'] >= 35 and self.status['motor_left'] == 'Stop')"
  - If we just want the Goalset to run without conditions, we need just one condition defined as "True"
- starts_at
  - This defines the time at which the Goalset is going to be triggered.
  - You can think about it as an "AND self.status['time'] >= X" added to each condition on conditions_or
  - I still have not removed starts_at in favor of self.status['time'] because it is used in some other formulas and I'd need to search for the exact String in your condition, which is a source for issues.
    - Sorry about that, I know it's lame.

Each one stores a set of goals or actions that will get done by the robot in the order they are defined.

Each Goalset can be repeated as a group, it can be run at a specific point in time, and it can have a delay added to that.

Each goal can itself define several actions that happen at the same time, separated by '|'. The definition of each single action follows the patter <element>=<goal>.

Example:
```
- name: main
  conditions_or:
    - "True"
  starts_at: 2
  start_delay: 0.5
  repeat_nr: -1
  goals:
    - do: "motorleft=Forward|motorright=Forward"
      time_secs: 0.5 
    - do: "motorleft=Backward|motorright=Backward"
      time_secs: 0.5 
    - do: "motorleft=Stop|motorright=Stop"
      time_secs: 0.5 
```
The above will, in this order:
- wait 2 seconds after the robot starts (starts_at) 
- then wait another 0.5 seconds (start_delay)
- then have motorleft try to move Forward AND motorright try to move Forward too (motorleft=Forward|motorright=Forward)
- after 0.5 seconds (time_secs) have motorleft try to move Backward AND motorright try to move Backward too (motorleft=Backward|motorright=Backward)
- after 1 seconds (time_secs) have motorleft try to Stop AND motorright try to Stop too (motorleft=Stop|motorright=Stop)
- after 2.5 seconds (time_secs) it will wait another 0.5 seconds (start_delay) and run another Repeat (repeat_nr) as follows:
- it will have motorleft try to move Forward AND motorright try to move Forward too (motorleft=Forward|motorright=Forward)
- after 0.5 seconds (time_secs) have motorleft try to move Backward AND motorright try to move Backward too (motorleft=Backward|motorright=Backward)
- after 1 seconds (time_secs) have motorleft try to Stop AND motorright try to Stop too (motorleft=Stop|motorright=Stop)
- after 2.5 seconds (time_secs) it will have finished its execution
