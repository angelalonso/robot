# Robot

Resources to build a home robot.


# WHY this guide?
- I have very specific Hardware, most notable an <old> Raspberry pi B.  
- Most of the docs online refer to the newer boards.  
- Everything I document here was either not-so-easy to find or directly a result of my trial and error.  

# Status

Under development

## Version 2020.09
### Hardware
- Raspberry pi 1 B+ as Brain runner and movement controller. Check [RASPBERRY.md](RASPBERRY.md)  
- Arduino UNO as sensor controller. Check [ARDUINO.md](ARDUINO.md) 
- L298N as motor controller. Check [L298N.md](L298N.md)  
- HC-SR04 as distance sensor  
- 2 motors  
- Chassis made of Cardboard and plastic  
- Weight: 895 grams  

### Connections
![Diagram as of September 2020](diagram.202009.png)

### Software
Check [BRAIN'S own README](brain/README.md)

### Challenges
- Sensor is imprecise  
- Sometimes the Brain process runs simultaneously  
- Weight should be lowered but anything under 1 Kg is fine I guess  
- It doesn't do much  
- I have smaller powerbanks but they dont work well with the L298N motor controller  

 
