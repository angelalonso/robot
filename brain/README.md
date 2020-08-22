# Current usage

## A -> Arduino program
- sudo arduino
- make arduino be verbose (TODO: Document how)
- create program
- save it to ../tests/name_of_program (or whatever folder you want)
- compile it locally 
- check output of arduino, copy the /tmp/../...ino.hex file to the ../tests/name_of_program folder
- chown the whole folder to your user

## B -> config file
- edit cfg.yaml
- For each action, add one of the following:
```
- trigger: "ping\r\n"     <- here \r\n needs to be added to what you expect your arduino hex to send you back
  actions:
    - sendfileserial_../tests/001_test_pong/001_test_pong.ino.hex  <- here you just put sendfileserial_<and the path to the next .ino.hex file you want to install to the arduino>
```


## Run tests
- run cp env.template .env
- modify .env to your needs 
- run ./test_on_raspberry.sh 


# ISSUES
- The arduino code cannot use more than one Serial.println(). It gets "stuck" (probably Brain, after reading from serial once it doesn't get anything new)
- One has to manually get and copy the ino.hex files after compiling from arduino IDE :(
