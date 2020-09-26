# Information about the L298N Motor Controller
## Connection to Motors, Power, Raspberry
```
               |_                       _|
MotorA, out 1--|O|                     |O|--MotorB, out 1
MotorA, out 2--|O|        _         _  |O|--MotorB, out 2
               |" _____  |x|       |x|  "|
               |_|O|O|x|_|O|O|O|O|O|O|___|
                  | |     | | | | | |
             Vcc--+ |     | | | | | +-EnaMotorB ---> GPIO25, pin 22 Raspberry
             GND----+     | | | | +---MotorB, in2 -> GPIO24, pin 18    " 
                          | | | +-----MotorB, in1 -> GPIO23, pin 16    " 
                          | | +-------MotorA, in2 -> GPIO27, pin 13    "
                          | +---------MotorA, in1 -> GPIO17, pin 11    "
                          +-----------EnaMotorA ---> GPIO22, pin 15    "

 ```
