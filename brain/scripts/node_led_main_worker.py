#!/usr/bin/env python3

from rclpy import init
from rclpy import spin
from rclpy import shutdown
from rclpy import logging
from rclpy.action import ActionServer
from rclpy.node import Node
import RPi.GPIO as GPIO
#from fake_rpi import fake_rpi as GPIO

from brain.action import Led

from dotenv import load_dotenv
from os import getenv

class LEDMainActionServer(Node):

    def __init__(self, PinNr, loglevel):
        super().__init__('ledmain_action_server')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        self.pin_id = PinNr
        self.turn_on = False
        self._action_server = ActionServer(
            self,
            Led,
            'LedMain',
            self.execute_callback)
        self.get_logger().fatal('STARTED LED')

    def execute_callback(self, goal_handle):
        feedback_msg = Led.Feedback()
        goal_handle.publish_feedback(feedback_msg)

        GPIO.setup(self.pin_id, GPIO.OUT, None)
        if self.turn_on == False:
            feedback_msg.process_feed = "setting PIN " + str(self.pin_id) + " On"
            GPIO.output(self.pin_id, GPIO.HIGH)
            self.turn_on = True
        else:
            feedback_msg.process_feed = "setting PIN " + str(self.pin_id) + " Off"
            GPIO.output(self.pin_id, GPIO.LOW)
            self.turn_on = False
        self.get_logger().fatal('Feedback: {}'.format(feedback_msg.process_feed))
        goal_handle.succeed()
        result = Led.Result()
        return result

def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    LEDMAIN_PIN = getenv('LEDMAIN_PIN')

    init(args=args)

    ledmain_action_server = LEDMainActionServer(LEDMAIN_PIN, LOGLEVEL)

    spin(ledmain_action_server)

    shutdown()

if __name__ == '__main__':
    main()
