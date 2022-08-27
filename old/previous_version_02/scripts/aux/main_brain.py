#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
import time

from brain.action import Led

class Led21ActionClient(Node):

    def __init__(self):
        super().__init__('led_action_client')
        self.pin_id = 21
        self._action_client = ActionClient(self, Led, 'Led' + str(self.pin_id)) # IMPORTANT: that last Led21 has to match the Led+<LedNumber> format of the server

    def send_goal(self):
        goal_msg = Led.Goal()

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)

def main(args=None):
    rclpy.init(args=args)
    action_client = Led21ActionClient()
    counter = 0
    while counter < 10:
        counter += 1
        future = action_client.send_goal()
        rclpy.spin_until_future_complete(action_client, future)
        time.sleep(1)
    

if __name__ == '__main__':
    main()
