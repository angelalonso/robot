import RPi.GPIO as GPIO                                                                                  
import time

servoPIN = 18
GPIO.setmode(GPIO.BCM)
GPIO.setup(servoPIN, GPIO.OUT)

p = GPIO.PWM(servoPIN, 500) # GPIO 17 for PWM with 50Hz
p.start(2.5) # Initialization
try:
  while True:
      print("first")
      for dc in range(400, 1010, 50):
          print(dc/10)
          p.ChangeDutyCycle(dc/10)
          time.sleep(0.5)
      print("second")
      for dc in range(1000, 350, -50):
          print(dc/10)
          p.ChangeDutyCycle(dc/10)
          time.sleep(0.5)
except KeyboardInterrupt:
  pass
print("cleaned")
p.stop()
GPIO.cleanup()

