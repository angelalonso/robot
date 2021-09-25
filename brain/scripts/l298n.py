#!/usr/bin/env python3

#import RPi.GPIO as GPIO          
from fake_rpi import fake_rpi as GPIO
from time import sleep

right_in1 = 24
right_in2 = 23
right_en = 25
right_state=1

GPIO.setmode(GPIO.BCM)
GPIO.setup(right_in1,GPIO.OUT,None)
GPIO.setup(right_in2,GPIO.OUT,None)
GPIO.setup(right_en,GPIO.OUT,None)
GPIO.output(right_in1,GPIO.LOW)
GPIO.output(right_in2,GPIO.LOW)
p=GPIO.PWM(right_en,1000)
p.start(100)
print("\n")
print("The default speed & direction of motor is LOW & Forward.....")
print("r-run s-stop f-forward b-backward l-low m-medium h-high e-exit")
print("\n")    

while(1):

    x=input()
    
    if x=='r':
        print("run")
        if(right_state==1):
         GPIO.output(right_in1,GPIO.HIGH)
         GPIO.output(right_in2,GPIO.LOW)
         print("forward")
         x='z'
        else:
         GPIO.output(right_in1,GPIO.LOW)
         GPIO.output(right_in2,GPIO.HIGH)
         print("backward")
         x='z'


    elif x=='s':
        print("stop")
        GPIO.output(right_in1,GPIO.LOW)
        GPIO.output(right_in2,GPIO.LOW)
        x='z'

    elif x=='f':
        print("forward")
        GPIO.output(right_in1,GPIO.HIGH)
        GPIO.output(right_in2,GPIO.LOW)
        right_state=1
        x='z'

    elif x=='b':
        print("backward")
        GPIO.output(right_in1,GPIO.LOW)
        GPIO.output(right_in2,GPIO.HIGH)
        right_state=0
        x='z'

    elif x=='l':
        print("low")
        p.ChangeDutyCycle(25)
        x='z'

    elif x=='m':
        print("medium")
        p.ChangeDutyCycle(50)
        x='z'

    elif x=='h':
        print("high")
        p.ChangeDutyCycle(99)
        x='z'
     
    
    elif x=='e':
        GPIO.cleanup()
        break
    
    else:
        print("<<<  wrong data  >>>")
        print("please enter the defined data to continue.....")

