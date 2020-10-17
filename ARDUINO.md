# Information about the Arduino part

For the Raspberry pi to install .hex programs into the Arduino you'll need to setup the ICSP connection between both.

After that, you'll need avrdude to be installed and configured on the Raspberry. 

The Brain program uses avrdude to get this part done.

## PINS on the Arduino clone "ISCP" Map - yeah I know, freaking clones' typos
```
-----------
  |1|3|5|  
  |2|4|6|  
  
1 Vcc  
2 MISO  
3 ICSP MOSI  
4 ICSP SCK  
5 ICSP GND  
6 ICSP RESET  
```

### Connection to Raspberry pi 1 B+ 
```
ISCP Pin NNumber -> Raspberry Pi Pin Number
               1 -> 2  
               2 -> 21  
               3 -> 19  
               4 -> 23  
               5 -> 6  
               6 -> 7  
```

# Writing "compatible" programs for the Arduino
## Programming workflow
- Install arduino in your computer.
- Run arduino.
- Make arduino verbose on compilation
  - File > Preferences > Show verbose output during compilation
- Create program
- Save it to ./arduino/name_of_program (or whatever folder you want)
- compile it locally 
- check output of arduino, copy the /tmp/../...ino.hex file to the ../arduino/name_of_program folder
- chown the whole folder to your user

## Notes on writing programs
Communication from Arduino to Raspberry occurs through messages on the Serial.
- There are three types of messages; actions, metrics and the rest, which should just be used as logs  
  - Action messages start with "ACTION: "
    - The rest of the message contains the trigger that will make brain take an action.
    - If you want your robot to do something, configure an entry on brain/cfg.yaml(copy and paste is your friend), and make your program send something like:
```
    delay(50);
    Serial.println("ACTION: talk_dirty");
```
  - Metrics messages start with "SENSOR: data\_"
    - The full message looks like "SENSOR: data\_distance\_20", where distance is a sensor brain recognizes and 20 is the value we got from that sensor.
    - The arduino part of sending that message looks like:
```
    delay(50);
    Serial.print("SENSOR: data_distance_");
    Serial.println(dist); # dist is defined previously and stores values directly from your sensor.
```
  - Information ones start with "LOG: "
- As you have seen on the examples above, we need to have a ```delay(50)``` (or more than 50) between two println commands, to allow brain to read both in two separate tries.

# Challenges
- One has to manually get and copy the ino.hex files after compiling from arduino IDE :(
