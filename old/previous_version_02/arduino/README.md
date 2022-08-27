# Arduino side of the robot
Since we are using ROS2 and an Arduino UNO, none of the rosserial/micro-ros/ros2arduino solutions to encapsulate a node seem to work.

Therefore, I am just using USB-Serial communication and pyserial to catch the messages within a Node that runs on the Raspberry.

## Requirements
- You must have arduino installed on your laptop

## Loading the program to your Arduino
- Open arduino
- Connect the Arduino to your Laptop thorugh USB
- Tools > Port > Choose the one where your Arduino is (e.g.: /dev/ttyACM0)
- Click on Upload


