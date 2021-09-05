#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
import time

from brain.action import Led21On, Led21Off

class Led21OnActionClient(Node):

    def __init__(self):
        super().__init__('led21on_action_client')
        self._action_client = ActionClient(self, Led21On, 'Led21On')

    def send_goal(self):
        goal_msg = Led21On.Goal()

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)

class Led21OffActionClient(Node):

    def __init__(self):
        super().__init__('led21off_action_client')
        self._action_client = ActionClient(self, Led21Off, 'Led21Off')

    def send_goal(self):
        goal_msg = Led21Off.Goal()

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)

def main(args=None):
    rclpy.init(args=args)
    action_client = Led21OnActionClient()
    action_client_off = Led21OffActionClient()
    counter = 0
    while counter < 10:
        counter += 1
        if (counter % 2 == 0):
            future = action_client.send_goal()
            rclpy.spin_until_future_complete(action_client, future)
        else:
            future2 = action_client_off.send_goal()
            rclpy.spin_until_future_complete(action_client_off, future2)
        time.sleep(1)
    

if __name__ == '__main__':
    main()
