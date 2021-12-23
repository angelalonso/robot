import rclpy
from rclpy.node import Node
import RPi.GPIO as GPIO

class MinimalTest(Node):

    def __init__(self):
        super().__init__('minimal_test')
        GPIO.setwarnings(False)
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(21, GPIO.OUT)
        GPIO.output(21, GPIO.LOW)


def main(args=None):
    rclpy.init(args=args)

    action_client = MinimalTest()

    rclpy.shutdown()


if __name__ == '__main__':
    main()

