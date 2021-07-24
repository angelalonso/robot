# Attaching a Camera to the Robot

Even though our Robot is designed to work within reach of a WLAN, it can be pretty useful to have a camera attached to it.

For this setup, I chose an ESP32-CAM module. This module does not interact with our Robot other than getting its power from it.

The ESP32-CAM setup requires an FTDI232 chip to program it.
  
## How to use this feature
## Shopping list
I am not adding these two elements to [the main shopping list](./000_ShoppingList.md) because they re not technically needed to build the robot AND they are not really integrated into it. Yet.
  
You'll need:
- [An ESP32 CAM](https://www.banggood.com/ESP32-CAM-WiFi-+-bluetooth-Camera-Module-Development-Board-ESP32-With-Camera-Module-OV2640-p-1394679.html?cur_warehouse=CN&rmmds=search)
- [An FTDI232 USB-to-Serial chip like these](https://www.amazon.com/s?k=ftdi232+usb+to+serial&crid=HFWAC7THRVAB&sprefix=FTDI232%2Caps%2C284&ref=nb_sb_ss_ts-doa-p_1_7). I bought [this one](https://www.ebay.de/itm/FT232RL-3-3V-5-5V-FTDI-USB-to-TTL-Serial-Adapter-Modul-for-Arduino-Mini-Port-/174087046369)
- Some Jumper wires
  
## Connect your FTDI232 to the ESP32-CAM
See [this](https://randomnerdtutorials.com/program-upload-code-esp32-cam/)
  
## Prepare the program for your FTDI232
Basically follow [this video tutorial from minutes 10:43 on](https://youtu.be/5XCb3t8J4Kg?t=643). Still, here are some notes from me (to be improved)
  
   
- Get [here](https://github.com/espressif/arduino-esp32/blob/master/docs/arduino-ide/boards_manager.md) to grap the latest stable json
- Open Arduino > File > Preferences, and add that json URL to "Additional Boards Manager URLs". Click OK
- Tools > Board:xxx > Boards Manager. Search for esp32 and install the latest one from Espressif
- Tools > Board:xxx > ESP32 Arduino > ESP32 Wrover Module
- Tools > Partition Scheme:xxx > Select Huge APP
(...) TO BE DONE
- Make sure your code only has ```#define CAMERA_MODEL_AI_THINKER``` enabled
- If it fails to compile, complaining about pyserial, make sure you install it for python2 (despite your main python version being python3)
  
## Connect FTDI to computer and push the program
- Nothing much to add, cable in on both ends and push the program from the Arduino IDE
  
## Remove connections and power your cam from the Robot
- Provided the Robot has the setup discussed along these docs, connect two cables from 5V and GND pins on the ESP32 CAM to the related points in the breadboard.

# Challenges
- It'd be great if one day the robot and the camera could do more together

[PREV: Using real Robot's input as mocks <--](008_TestingWithRealMocks.md)
