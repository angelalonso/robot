#!/usr/bin/env python3

from interfaces.srv import GetStatus, GetStatusKey, SetStatus

from rclpy import init, logging, spin, shutdown
from rclpy.action import ActionServer, GoalResponse
from rclpy.node import Node
from rclpy.callback_groups import ReentrantCallbackGroup
from rclpy.executors import MultiThreadedExecutor

from brain.action import Getstatus
from service_status import Status

import flatdict
from dotenv import load_dotenv
from os import getenv
import time

#------
from rclpy import executors, ok
import threading
#------

class StatusActionServer(Node):
    def __init__(self, loglevel):
        super().__init__('status_action_server')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        self.status = Status()

       # self._action_server = ActionServer(
       #     self,
       #     Getstatus,
       #     'GetStatusServo',
       #     self.execute_callback)
        self._action_server = ActionServer(
            self,
            Getstatus,
            'GetStatusServo',
            execute_callback=self.execute_callback,
            callback_group=ReentrantCallbackGroup(),
            goal_callback=self.goal_callback,
            cancel_callback=self.cancel_callback)

    def goal_callback(self, goal_request):
        """Accept or reject a client request to begin an action."""
        # This server allows multiple goals in parallel
        self.get_logger().info('Received goal request')
        return GoalResponse.ACCEPT

    def cancel_callback(self, goal_handle):
        """Accept or reject a client request to cancel an action."""
        self.get_logger().info('Received cancel request')
        return CancelResponse.ACCEPT

    async def execute_callback(self, goal_handle):
        feedback_msg = Getstatus.Feedback()
        goal_handle.publish_feedback(feedback_msg)

        callback_type = goal_handle.request.key.split('_')[0]
        callback_data = goal_handle.request.key.split('_')[1]

        goal_handle.succeed()
        result = Getstatus.Result()
        if callback_type == 'get':
            try:
                result.value = self.status[callback_data]
            except KeyError:
                result.value = 'nothing'
        if callback_type == 'set':
            this_key = callback_data.split('=')[0]
            this_value = callback_data.split('=')[1]
            self.status.set_status(this_key, this_value)
            result.value = ''

        time.sleep(1)
        self.get_logger().info('....in node_status')
        return result



def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')

    init(args=args)

    service_status = StatusActionServer(LOGLEVEL)
    executor = MultiThreadedExecutor()

    spin(service_status, executor=executor)

    service_status.destroy()

    #spin(service_status)

    shutdown()

if __name__ == '__main__':
    main()
