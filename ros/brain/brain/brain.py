import rclpy
from rclpy.node import Node
import RPi.GPIO as GPIO

class BrainMain(Node):

    def __init__(self):
        super().__init__('brain_main')
        GPIO.setwarning(False)
        GPIO.setmode(GPIO.BCM)
        # TODO: read pin config file and fix pins setup 

    def led(pin_id, state):
        if state == 'on':
            GPIO.setup(pin_id, GPIO.OUT)
            GPIO.output(pin_id, GPIO.HIGH)
        elif state == 'off':
            GPIO.setup(pin_id, GPIO.OUT)
            GPIO.output(pin_id, GPIO.LOW)
        else:
            # TODO: use logging instead
            print("ERROR: Wrong state")

def main(args=None):
    rclpy.init(args=args)
    # TODO: turn pin 21 on and off in a loop every second
    # TODO: do it after input, move input outside this script
    rclpy.shutdown()

if __name__ == '__main__':
    main()
