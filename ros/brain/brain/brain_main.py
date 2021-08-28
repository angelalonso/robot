import rclpy
from rclpy.node import Node
import RPi.GPIO as GPIO
#from fake_rpi import fake_rpi as GPIO
import signal
import sys
import time

def signal_handler(sig, frame):
    print('You pressed Ctrl+C!')
    rclpy.shutdown()
    sys.exit(0)


class BrainMain(Node):

    def __init__(self):
        super().__init__('brain_main')
        GPIO.setwarning(False)
        GPIO.setmode(GPIO.BCM)
        # TODO: read pin config file and fix pins setup 

    def led(self, pin_id, state):
        print("state is " + str(state))
        if state:
            GPIO.setup(pin_id, GPIO.OUT)
            GPIO.output(pin_id, GPIO.HIGH)
        else:
            GPIO.setup(pin_id, GPIO.OUT)
            GPIO.output(pin_id, GPIO.LOW)

def main(args=None):
    rclpy.init(args=args)
    # TODO: do it after input, move input outside this script
    signal.signal(signal.SIGINT, signal_handler)
    # TODO: use logging instead
    print('Press Ctrl+C')
    loop = True
    brain = BrainMain()
    led_state = False
    while loop:
        if led_state:
            led_state = False
        else:
            led_state = True

        brain.led("21", led_state)
        time.sleep(1)

if __name__ == '__main__':
    main()
