# Attaching a Camera to the Robot

Even though our Robot is designed to work within reach of a WLAN, it can be pretty useful to have a camera attached to it.

For this setup, I chose an ESP32-CAM module. This module does not interact with our Robot other than getting its power from it.

The ESP32-CAM setup requires an FTDI232 chip to program it.

## How to use this feature
- Connect your FTDI232 to the ESP32-CAM
- Prepare the program for your FTDI232
- Connect FTDI to computer and push the program
- Remove connections and power your cam from the Robot

# Challenges
- It'd be great if one day the robot and the camera could do more together

[PREV: Using real Robot's input as mocks <--](008_TestingWithRealMocks.md)
