#!/usr/bin/env python3

from rclpy import logging, ok, spin_once
from rclpy.node import Node
from rclpy.action import ActionClient

from brain.action import Motor, Servo, Getstatus
from interfaces.srv import GetStatusKey, SetStatus

import asyncio

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

class GetStatusKeyActionClient(Node):

    def __init__(self):
        super().__init__('get_status_key_action_client')
        self.getstatuskey_cli = self.create_client(GetStatusKey, 'getstatuskey')
        while not self.getstatuskey_cli.wait_for_service(timeout_sec=1.0):
            self.get_logger().info('service not available, waiting again...')
        self.getstatuskey_req = GetStatusKey.Request()

    def send_getstatuskey_req(self, key):
        self.getstatuskey_req.key = key
        self.future = self.getstatuskey_cli.call_async(self.getstatuskey_req)

    def send_getstatuskey(self, key):
        self.send_getstatuskey_req(key)
        while ok():
            spin_once(self)
            if self.future.done():
                try:
                    response = self.future.result()
                except Exception as e:
                    self.get_logger().debug('Service call failed %r' % (e,))
                else:
                    result = response.current_status
                break
        return result


class ServoLaserActionClient(Node):

    def __init__(self):
        super().__init__('servo_laser_action_client')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, 'INFO'))
        self._action_client = ActionClient(self, Servo, 'ServoLaser')

    def send_goal(self, rotation):
        goal_msg = Servo.Goal()
        goal_msg.rotation = float(rotation)

        self._action_client.wait_for_server()

        #return self._action_client.send_goal_async(goal_msg)

        self._send_goal_future = self._action_client.send_goal_async(
            goal_msg,
            feedback_callback=self.feedback_callback)

        self._send_goal_future.add_done_callback(self.goal_response_callback) 
        self.get_logger().info('Goal Send called ')
        return 

    def goal_response_callback(self, future):
        goal_handle = future.result()

        if not goal_handle.accepted:
            self.get_logger().info('Goal rejected :(')
            return

        self.get_logger().info('Goal accepted :)')

        self._get_result_future = goal_handle.get_result_async()

        self._get_result_future.add_done_callback(self.get_result_callback)

        return self._get_result_future

    def get_result_callback(self, future):
        result = future.result().result
        self.get_logger().info('Result: {0}'.format(result.value))
        rclpy.shutdown()

    def feedback_callback(self, feedback_msg):
        feedback = feedback_msg.feedback
        self.get_logger().info('Received feedback: {0}'.format(feedback.partial_sequence))

