# Adding a new Input to our Robot

After the robot can follow a set of goals/orders/actions, the next step is to add Inputs that can affect which goal(s) come next.

## Arduino UNO

In our current design, an Arduino serves gets data from all inputs and sends that in a regular interval to the Raspberry.

- Attach it to the upper layer of the chassis.
- You will later use the USB cable to link the Arduino to the Raspberry when you are ready
  - Power and Communication to Raspberry will happen through that USB cable
- But first, connect it to your laptop 
- Open arduino/sensors/sensors.ino with the arduino IDE
  - Load the program to the connected Arduino
- Disconnect the USB Cable from your laptop and connect it to the Raspberry
    
I could probably use a different model, but this was the one I had at home.

## Breadboard

We are adding a breadboard to have a flexible connection set between elements.
- Attach the breadboard to the upper layer of the chassis.

One of the most important uses is to have a Common Ground with L298N among others.

## Ultrasonic Distance Sensor - HC-SR04 

You can buy one here https://www.sparkfun.com/products/15569

It will receive Power from and communicate to the Arduino using some jumper wires.
- Set the cables as follows:
```
               |                 |
               |____|O|O|O|O|____|
                     | | | |
                     | | | | 
                     | | | |
                     | | | +-----Vcc     -> Arduino    5v PIN
                     | | +-------Trigger -> Arduino PWM 3 PIN
                     | +---------Echo    -> Arduino PWM 2 PIN
                     +-----------Ground  -> Arduino   GND PIN

 ```
- This is where the breadboard comes in handy:
  - Instead of connecting 5v and GND directly, use the + and - lines on the breadboard as intermediary.
  - We will connect more stuff to these lines in the future

Now you can modify brain/goalsets/main.yaml to add the values you get from the Distance sensor as a condition.
E.g.: You can make your robot stop when there's something closer to 35 cms in front of it with:
 ```
(...)
conditions_or: 
  - "(self.status['distance'] <= 35)"
(...)
goals:
  - do: "motorleft=Stop|motorright=Stop"
    time_secs: 0.5 
 ```
