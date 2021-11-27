#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from rclpy.logging import LoggingSeverity
from rclpy.action import ActionClient
from std_msgs.msg import Int32
from std_msgs.msg import String

import os

import time
import threading

from brain.action import Motor 

var=None

LOGLEVEL="debug"

#define the display text
def callback(msg):
    global var
    var=msg.data


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


class MainTopicSubscriber(Node):

    def __init__(self):
        super().__init__('main_topic_subscriber')
        self.subscription = self.create_subscription(
            String,
            'main_topic',
            self.listener_callback,
            10)
        self.subscription  # prevent unused variable warning
        self.left_motor = LeftMotorActionClient()
        self.right_motor = RightMotorActionClient()

    def from_main_to_actions(self, action_list):
        for action in action_list:
            if action.split('=')[0] == "motor_left":
                self.left_motor.send_goal(action.split('=')[1])
            elif action.split('=')[0] == "motor_right":
                self.right_motor.send_goal(action.split('=')[1])
            self.get_logger().debug('ACTION: "%s"' % action)

    def listener_callback(self, msg):
        self.get_logger().debug('I heard: "%s"' % msg.data)
        # check this has ACTIONS indeed
        self.from_main_to_actions(msg.data.replace("ACTIONS: ", "").split('|'))

def main(args=None):
    rclpy.init(args=args)

    minimal_subscriber = MainTopicSubscriber()

    rclpy.spin(minimal_subscriber)

    minimal_subscriber.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()
