# Current usage

## A -> Arduino program
- sudo arduino
- make arduino be verbose (TODO: Document how)
- create program
- save it to ../tests/name_of_program (or whatever folder you want)
- compile it locally 
- check output of arduino, copy the /tmp/../...ino.hex file to the ../tests/name_of_program folder
- chown the whole folder to your user

# B -> config file

# ISSUES
- The arduino code cannot use more than one Serial.println(). It gets "stuck" (probably Brain, after reading from serial once it doesn't get anything new)
- One has to manually get and copy the ino.hex files after compiling from arduino IDE :(
