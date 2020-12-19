# Using your own real Robot's input as mocks

It is possible for you to run your code on your robot, and save the sensors inputs to use them offline.

This is not very useful when we only have a distance sensor to capture, but we assume our Robot will become more and more advanced until, well, the singularity.

## How to use this feature
- Modify  this line in do_record.sh:
```
SETUP_FILE="setup.yaml"
```

, to the path of your setup file, probably called something like testfiles/yourtestname_setup.yaml if you follow the naming convention we use.
- Position your robot where you know the result will make sense (e.g.: close to an obstacle, over a line it should follow...) and run:
```
./do_record.sh testfiles/yourtestname_mock.yaml ; ./do_reset.sh
```
, where `testfiles/yourtestname_mock.yaml` should, again, probably fit the naming convention of a test you are already working on.
Also, the ./do_reset.sh script is included there to make sure the Robot stops at some point.
- When you're happy with the result, press <CTRL+C> to make it stop and send the mock file over to your computer.
- Last but not least: check your mock file and feel free to modify any entries that might look weird.

# Challenges
- This whole process, among others, would benefit from a central script that runs all possible modes.

[PREV: Testing a movement Ruleset <--](007_TestingExample.md)
