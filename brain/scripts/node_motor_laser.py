import RPi.GPIO as GPIO
import time

servoPIN = 18
GPIO.setmode(GPIO.BCM)
GPIO.setup(servoPIN, GPIO.OUT)

p = GPIO.PWM(servoPIN, 50) # GPIO 17 for PWM with 50Hz
correction = -1.25
p.start(2.5 + correction) # Initialization
try:
  while True:
    p.ChangeDutyCycle(5 + correction)
    time.sleep(0.5)
    p.ChangeDutyCycle(7.5 + correction)
    time.sleep(0.5)
    p.ChangeDutyCycle(10 + correction)
    time.sleep(0.5)
    p.ChangeDutyCycle(12.5 + correction)
    time.sleep(0.5)
    p.ChangeDutyCycle(10 + correction)
    time.sleep(0.5)
    p.ChangeDutyCycle(7.5 + correction)
    time.sleep(0.5)
    p.ChangeDutyCycle(5 + correction)
    time.sleep(0.5)
    p.ChangeDutyCycle(2.5 + correction)
    time.sleep(0.5)
except KeyboardInterrupt:
  p.stop()
  GPIO.cleanup()

