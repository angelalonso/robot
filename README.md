# Robot

Resources to build a home robot.


# WHY this guide?
- I have very specific Hardware, most notable an <old> Raspberry pi B.  
- Most of the docs online refer to the newer boards.  
- Everything I document here was either not-so-easy to find or directly a result of my trial and error.  

# Status

Under development

## Version 2020.09
- Raspberry pi 1 B+ as Brain runner and movement controller. Check [RASPBERRY.md](RASPBERRY.md)  
- Arduino UNO as sensor controller
- L298N as motor controller
- HC-SR04 as distance sensor
- 2 motors

# FROM HERE ON, THE DOCS ARE OBSOLETE/NOT REVIEWED

# PHASE 1: Connect Raspberry to Arduino
## GPIO Map for Raspberry pi 1 REV2 Model B 

```
           .___.              
    5v---1-|O O|--2--3.3v  
    5v---3-|O O|--4--2 SDA  
   GND---5-|O O|--6--3 SCL  
14 TXD---7-|O O|--8--4  
15 RXD---9-|O O|-10--GND  
    18--11-|O O|-12--17  
   GND--13-|O O|-14--27  
    23--15-|O O|-16--22  
    24--17-|O O|-18--3.3v  
   GND--19-|O O|-20--10 MOSI  
    25--21-|O O|-22--9 MISO  
     8--23-|O O|-24--11 SCKL  
     7--25-|O O|-26--GND  
           '---'

```
## GPIO Map for Raspberry pi 1 B+ 
(thanks to https://github.com/tvierb/raspberry-ascii)
```
                           .___.              
                  +3V3---1-|O O|--2--+5V
          (SDA)  GPIO2---3-|O O|--4--+5V
         (SCL1)  GPIO3---5-|O O|--6--_
    (GPIO_GLCK)  GPIO4---7-|O O|--8-----GPIO14 (TXD0)
                      _--9-|O.O|-10-----GPIO15 (RXD0)
    (GPIO_GEN0) GPIO17--11-|O O|-12-----GPIO18 (GPIO_GEN1)
    (GPIO_GEN2) GPIO27--13-|O O|-14--_
    (GPIO_GEN3) GPIO22--15-|O O|-16-----GPIO23 (GPIO_GEN4)
                  +3V3--17-|O O|-18-----GPIO24 (GPIO_GEN5)
     (SPI_MOSI) GPIO10--19-|O.O|-20--_
     (SPI_MOSO) GPIO9 --21-|O O|-22-----GPIO25 (GPIO_GEN6)
     (SPI_SCLK) GPIO11--23-|O O|-24-----GPIO8  (SPI_C0_N)
                      _-25-|O O|-26-----GPIO7  (SPI_C1_N)
       (EEPROM) ID_SD---27-|O O|-28-----ID_SC Reserved for ID EEPROM
                GPIO5---29-|O.O|-30--_
                GPIO6---31-|O O|-32-----GPIO12
                GPIO13--33-|O O|-34--_
                GPIO19--35-|O O|-36-----GPIO16
                GPIO26--37-|O O|-38-----GPIO20
                      _-39-|O O|-40-----GPIO21
                           '---'
(_ means Ground)
```

## Arduino clone "ISCP" Map - yeah I know, freaking clones' typos
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

## Connection
### Arduino -> Raspberry pi 1 REV2 Model B 
TBD

### Arduino -> Raspberry pi 1 B+ 
```
1 -> 2  
2 -> 21  
3 -> 19  
4 -> 23  
5 -> 6  
6 -> 7  
```

### L298N -> Motors, Power, Raspberry
```
                _                       _
MotorA, out 1--|O|                     |O|--MotorB, out 1
MotorA, out 2--|O|        _         _  |O|--MotorB, out 2
                "        |x|       |x|  "
                 |O|O|x| |O|O|O|O|O|O|
                  | |     | | | | | |
             Vcc--+ |     | | | | | +-EnaMotorB ---> GPIO25, pin 22
             GND----+     | | | | +---MotorB, in2 -> GPIO24, pin 18
                          | | | +-----MotorB, in1 -> GPIO23, pin 16 
                          | | +-------MotorA, in2 -> GPIO27, pin 13
                          | +---------MotorA, in1 -> GPIO17, pin 11
                          +-----------EnaMotorA ---> GPIO22, pin 15

 ```
 
