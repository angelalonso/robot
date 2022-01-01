#!/usr/bin/env python3

from rclpy.node import Node
from rclpy.action import ActionClient

from brain.action import Motor, Servo

class MotorLeftActionClient(Node):

    def __init__(self):
        super().__init__('motor_left_action_client')
        self._action_client = ActionClient(self, Motor, 'MotorLeft')

    def send_goal(self, move):
        goal_msg = Motor.Goal()
        goal_msg.move = move

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)

class MotorRightActionClient(Node):

    def __init__(self):
        super().__init__('motor_right_action_client')
        self._action_client = ActionClient(self, Motor, 'MotorRight')

    def send_goal(self, move):
        goal_msg = Motor.Goal()
        goal_msg.move = move

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)

class ServoLaserActionClient(Node):

    def __init__(self):
        super().__init__('servo_laser_action_client')
        self._action_client = ActionClient(self, Servo, 'ServoLaser')

    def send_goal(self, rotation):
        goal_msg = Servo.Goal()
        goal_msg.rotation = float(rotation)

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)

