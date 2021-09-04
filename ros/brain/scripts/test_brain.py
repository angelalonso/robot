#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
import time

from brain.action import Led21On

def callAction(counter):
    print("Action called, ", counter)

class Led21OnActionClient(Node):

    def __init__(self):
        super().__init__('led21on_action_client')
        self._action_client = ActionClient(self, Led21On, 'Led21On')

    def send_goal(self, order):
        goal_msg = Led21On.Goal()

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)

def main(args=None):
    rclpy.init(args=args)
    action_client = Led21OnActionClient()
    counter = 0
    while counter < 10:
        counter += 1
        callAction(counter)
        future = action_client.send_goal("turn_on: True")
        rclpy.spin_until_future_complete(action_client, future)
        time.sleep(1)
    

if __name__ == '__main__':
    main()
