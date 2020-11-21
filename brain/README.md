# Brain

Program to manage the Robot.

The program manages: 
- Communications with Arduino
- Installing new .hex files into the Arduino
- Movement of the motors

## Configuration

Check [the setup and actions README](./SETUP_README.md)

## Run the code
You will start your robot from the laptop once it is up, running and connected to your wifi.  
I am assuming you know your robot's IP (if not, google).  
- Copy the env template to generate your own
```
cp env.template .env
```
- Edit and fill up .env to your needs.  
- Run the remote script
```
./install_and_run.sh
```
  - You might want to check and modify that script for things like changing the log level.
- Honestly, at this point you can always use cargo build and call the exec file at your Raspberry directly.

# Challenges
- V2 ended up being a cleanup and improving version. V3 should indeed aim for a callibration system.
- Event driven but only up to one layer. We will switch to skills that use the above callibration system.
- We need a way for the robot to load Brain on bootup. This should include an automated update if necessary.  
- We need to document or automate finding out the Robot's IP.

