# Current usage

## A -> Arduino program
- sudo arduino
- make arduino be verbose (TODO: Document how)
- create program
- save it to ../tests/name_of_program (or whatever folder you want)
- compile it locally 
- check output of arduino, copy the /tmp/../...ino.hex file to the ../tests/name_of_program folder
- chown the whole folder to your user

### Notes on writing programs
- There are two types of messages; information and Result  
  - Information ones start with "LOG:", they allow the program to continue.
  - The rest are taken as result, and from the brain's perspective, the program finishes there  
E.g:
  This one will finish after the message "j is 0"  
```
  int j;
  for ( j = 0; j < 10; j++) {
    Serial.print("j is ");
    Serial.println(j);
  }
```
  This one will send "LOG: j is 0", then "LOG: j is 1"... and so on  
```
  int j;
  for ( j = 0; j < 10; j++) {
    Serial.print("j is ");
    Serial.println(j);
  }
```
- We need to have a ```delay(50)``` (or more than 50) between two println, to allow brain to read both in two separate tries.

## B -> config file
- edit cfg.yaml
- For each action, add one of the following:
```
- trigger: "ping"
  actions:
    - install_../tests/001_test_pong/001_test_pong.ino.hex  <- here you just put install_<and the path to the next .ino.hex file you want to install to the arduino>
```


## Run tests
- run cp env.template .env
- modify .env to your needs 
- run ./test_on_raspberry.sh 


# ISSUES
- One has to manually get and copy the ino.hex files after compiling from arduino IDE :(
