import RPi.GPIO as GPIO                                                                                  
import time

servoPIN = 18
GPIO.setmode(GPIO.BCM)
GPIO.setup(servoPIN, GPIO.OUT)
 
p = GPIO.PWM(servoPIN, 500) # GPIO 17 for PWM with 50Hz
p.start(0) # Initialization
try:
  while True:
      print("first")
      for dc in range(0, 101, 5):
          p.ChangeDutyCycle(dc)
          time.sleep(0.5)
      print("second")
      for dc in range(100, 5, -5):
          p.ChangeDutyCycle(dc)
          time.sleep(0.5)
except KeyboardInterrupt:
  pass
print("cleaned")
p.stop()
GPIO.cleanup()

