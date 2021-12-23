#!/usr/bin/env python3

from rclpy import init
from rclpy import spin
from rclpy import shutdown
from rclpy import logging
from rclpy.action import ActionServer
from rclpy.node import Node
#import RPi.GPIO as GPIO
from fake_rpi import fake_rpi as GPIO

from brain.action import Motor

from dotenv import load_dotenv
from os import getenv

class RightMotorActionServer(Node):

    def __init__(self, loglevel, power_factor):
        super().__init__('rightmotor_action_server')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        self.right_in1 = 24
        self.right_in2 = 23
        self.right_en = 25
        self.state = "Stop"
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(self.right_in1,GPIO.OUT)
        GPIO.setup(self.right_in2,GPIO.OUT)
        GPIO.setup(self.right_en,GPIO.OUT)
        GPIO.output(self.right_in1,GPIO.LOW)
        GPIO.output(self.right_in2,GPIO.LOW)

        self.p = GPIO.PWM(self.right_en,1000)
        self.p.start(100 * power_factor)

        self._action_server = ActionServer(
            self,
            Motor,
            'RightMotor',
            self.execute_callback)

    def execute_callback(self, goal_handle):
        feedback_msg = Motor.Feedback()
        goal_handle.publish_feedback(feedback_msg)

        if goal_handle.request.move == "Forward":
            feedback_msg.process_feed = "moving Right Motor " + str(goal_handle.request.move)
            GPIO.output(self.right_in1,GPIO.LOW)
            GPIO.output(self.right_in2,GPIO.HIGH)
            print("forward")
        elif goal_handle.request.move == "Backward":
            feedback_msg.process_feed = "moving Right Motor " + str(goal_handle.request.move)
            GPIO.output(self.right_in1,GPIO.HIGH)
            GPIO.output(self.right_in2,GPIO.LOW)
        else:
            feedback_msg.process_feed = "NOT moving Right Motor because movement: " + str(goal_handle.request.move)
            GPIO.output(self.right_in1,GPIO.LOW)
            GPIO.output(self.right_in2,GPIO.LOW)

        if self.state != goal_handle.request.move:
            self.state = goal_handle.request.move
            self.get_logger().info('Feedback: {}'.format(feedback_msg.process_feed))

        goal_handle.succeed()
        result = Motor.Result()
        return result

def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    MOTOR_R_FACTOR = float(getenv('MOTOR_R_FACTOR'))

    init(args=args)

    rightmotor_action_server = RightMotorActionServer(LOGLEVEL, MOTOR_R_FACTOR)

    spin(rightmotor_action_server)

    shutdown()

if __name__ == '__main__':
    main()
