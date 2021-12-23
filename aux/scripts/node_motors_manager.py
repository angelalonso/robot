#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
import time

from brain.action import Motor 

class RightMotorActionClient(Node):

    def __init__(self):
        super().__init__('right_motor_action_client')
        self._action_client = ActionClient(self, Motor, 'RightMotor')

    def send_goal(self, move):
        goal_msg = Motor.Goal()
        goal_msg.move = move

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)

class LeftMotorActionClient(Node):

    def __init__(self):
        super().__init__('left_motor_action_client')
        self._action_client = ActionClient(self, Motor, 'LeftMotor')

    def send_goal(self, move):
        goal_msg = Motor.Goal()
        goal_msg.move = move

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)

def main(args=None):
    rclpy.init(args=args)
    right_motor_client = RightMotorActionClient()
    left_motor_client = LeftMotorActionClient()
    counter = 0
    while counter < 10:
        counter += 1
        if counter % 3 == 0:
            movement = "Forward"
        elif counter % 3 == 1:
            movement = "Backward"
        else:
            movement = "Stop"
        right_future = right_motor_client.send_goal(movement)
        left_future = left_motor_client.send_goal(movement)
        rclpy.spin_until_future_complete(right_motor_client, right_future, executor=None, timeout_sec=1)
        rclpy.spin_until_future_complete(left_motor_client, left_future, executor=None, timeout_sec=1)
        time.sleep(1)
    

if __name__ == '__main__':
    main()
