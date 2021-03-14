# Using your own real Robot's input as mocks

It is possible for you to run your code on your robot, and save the sensors inputs to use them offline.

This is not very useful when we only have a distance sensor to capture, but we assume our Robot will become more and more advanced until, well, the singularity.

## How to use this feature
Get the path of your setup file, probably called something like testfiles/yourtestname_setup.yaml if you follow the naming convention we use.
- Position your robot where you know the result will make sense (e.g.: close to an obstacle, over a line it should follow...) and run:
```
./roctl do record testfiles/yourtestname_mock.yaml
```
, where `testfiles/yourtestname_mock.yaml` should, again, probably fit the naming convention of a test you are already working on.
- When you're happy with the result, press <CTRL+C> to make it stop and send the mock file over to your computer.
- Last but not least: check your mock file and feel free to modify any entries that might look weird.

# Challenges

[PREV: Testing a movement Ruleset <--](007_TestingExample.md) [--> NEXT: Attaching a Camera](009_AttachingCamera.md)
