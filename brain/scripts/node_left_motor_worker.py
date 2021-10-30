#!/usr/bin/env python3

import rclpy
from rclpy.action import ActionServer
from rclpy.node import Node
import RPi.GPIO as GPIO
#from fake_rpi import fake_rpi as GPIO

from brain.action import Motor

class LeftMotorActionServer(Node):

    def __init__(self):
        super().__init__('leftmotor_action_server')
        self.left_in1 = 27
        self.left_in2 = 17
        self.left_en = 22
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(self.left_in1,GPIO.OUT)
        GPIO.setup(self.left_in2,GPIO.OUT)
        GPIO.setup(self.left_en,GPIO.OUT)
        GPIO.output(self.left_in1,GPIO.LOW)
        GPIO.output(self.left_in2,GPIO.LOW)

        self.p = GPIO.PWM(self.left_en,1000)
        self.p.start(100)

        self._action_server = ActionServer(
            self,
            Motor,
            'LeftMotor',
            self.execute_callback)

    def execute_callback(self, goal_handle):
        feedback_msg = Motor.Feedback()
        goal_handle.publish_feedback(feedback_msg)

        if goal_handle.request.move == "Forward":
            feedback_msg.process_feed = "moving Left Motor " + str(goal_handle.request.move)
            GPIO.output(self.left_in1,GPIO.LOW)
            GPIO.output(self.left_in2,GPIO.HIGH)
            print("forward")
        elif goal_handle.request.move == "Backward":
            feedback_msg.process_feed = "moving Left Motor " + str(goal_handle.request.move)
            GPIO.output(self.left_in1,GPIO.HIGH)
            GPIO.output(self.left_in2,GPIO.LOW)
        else:
            feedback_msg.process_feed = "NOT moving Left Motor because movement: " + str(goal_handle.request.move)
            GPIO.output(self.left_in1,GPIO.LOW)
            GPIO.output(self.left_in2,GPIO.LOW)

        self.get_logger().info('Feedback: {}'.format(feedback_msg.process_feed))
        goal_handle.succeed()
        result = Motor.Result()
        return result

def main(args=None):
    rclpy.init(args=args)

    leftmotor_action_server = LeftMotorActionServer()

    rclpy.spin(leftmotor_action_server)

    rclpy.shutdown()

if __name__ == '__main__':
    main()
