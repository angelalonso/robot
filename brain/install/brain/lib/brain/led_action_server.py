#!/usr/bin/env python3

import rclpy
from rclpy.action import ActionServer
from rclpy.node import Node
#import RPi.GPIO as GPIO
from fake_rpi import fake_rpi as GPIO

from brain.action import Led


class LedActionServer(Node):

    def __init__(self, LedNr):
        super().__init__('led' + str(LedNr) +'_action_server')
        GPIO.setmode(GPIO.BCM)
        self.pin_id = LedNr
        self.turn_on = False
        self._action_server = ActionServer(
            self,
            Led,
            'Led' + str(self.pin_id),
            self.execute_callback)

    def execute_callback(self, goal_handle):
        feedback_msg = Led.Feedback()
        goal_handle.publish_feedback(feedback_msg)

        GPIO.setup(self.pin_id, GPIO.OUT)
        if self.turn_on == False:
            feedback_msg.process_feed = "setting PIN " + str(self.pin_id) + " On"
            GPIO.output(self.pin_id, GPIO.HIGH)
            self.turn_on = True
        else:
            feedback_msg.process_feed = "setting PIN " + str(self.pin_id) + " Off"
            GPIO.output(self.pin_id, GPIO.LOW)
            self.turn_on = False
        self.get_logger().info('Feedback: {}'.format(feedback_msg.process_feed))
        goal_handle.succeed()
        result = Led.Result()
        return result


def main(args=None):
    rclpy.init(args=args)

    led21_action_server = LedActionServer(21)

    rclpy.spin(led21_action_server)


if __name__ == '__main__':
    main()
