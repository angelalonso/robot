#!/usr/bin/env python3

import rclpy
from rclpy.action import ActionServer
from rclpy.node import Node
import RPi.GPIO as GPIO
#from fake_rpi import fake_rpi as GPIO

from brain.action import Led21On


class Led21OnActionServer(Node):

    def __init__(self):
        super().__init__('led21on_action_server')
        GPIO.setmode(GPIO.BCM)
        self.pin_id = 21
        self._action_server = ActionServer(
            self,
            Led21On,
            'Led21On',
            self.execute_callback)

    def execute_callback(self, goal_handle):
        self.get_logger().info('Executing goal...')

        feedback_msg = Led21On.Feedback()
        feedback_msg.process_feed = "handling PIN " + str(self.pin_id)
        self.get_logger().info('Feedback: {}'.format(feedback_msg.process_feed))
        goal_handle.publish_feedback(feedback_msg)

        GPIO.setup(self.pin_id, GPIO.OUT)
        GPIO.output(self.pin_id, GPIO.HIGH)
        goal_handle.succeed()
        result = Led21On.Result()
        return result


def main(args=None):
    rclpy.init(args=args)

    led21on_action_server = Led21OnActionServer()

    rclpy.spin(led21on_action_server)


if __name__ == '__main__':
    main()
