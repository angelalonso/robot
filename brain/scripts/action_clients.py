#!/usr/bin/env python3

from rclpy import logging, shutdown
from rclpy.node import Node
from rclpy.action import ActionClient

from brain.action import Led, Motor, Servo, Getstatus

import asyncio

class LedActionClient(Node):

    def __init__(self):
        super().__init__('led_action_client')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, 'INFO'))
        self._action_client = ActionClient(self, Led, 'LedMain')

    def send_goal(self, turn_on):
        goal_msg = Led.Goal()
        goal_msg.turn_on = turn_on

        self._action_client.wait_for_server()

        self._send_goal_future = self._action_client.send_goal_async(
            goal_msg,
            feedback_callback=self.feedback_callback)

        self._send_goal_future.add_done_callback(self.goal_response_callback) 
        self.get_logger().debug('Goal Send called ')
        return 

    def goal_response_callback(self, future):
        goal_handle = future.result()

        if not goal_handle.accepted:
            self.get_logger().debug('Goal rejected :(')
            return

        self.get_logger().debug('Goal accepted :)')

        self._get_result_future = goal_handle.get_result_async()

        self._get_result_future.add_done_callback(self.get_result_callback)

        return self._get_result_future

    def get_result_callback(self, future):
        result = future.result().result
        self.get_logger().debug('Result: {0}'.format(result.value))
        rclpy.shutdown()

    def feedback_callback(self, feedback_msg):
        feedback = feedback_msg.feedback
        self.get_logger().debug('Received feedback: {0}'.format(feedback.partial_sequence))


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
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, 'INFO'))
        self._action_client = ActionClient(self, Servo, 'ServoLaser')

    def send_goal(self, rotation):
        goal_msg = Servo.Goal()
        goal_msg.rotation = float(rotation)

        self._action_client.wait_for_server()

        self._send_goal_future = self._action_client.send_goal_async(
            goal_msg,
            feedback_callback=self.feedback_callback)

        self._send_goal_future.add_done_callback(self.goal_response_callback) 
        self.get_logger().debug('Goal Send called ')
        return 

    def goal_response_callback(self, future):
        goal_handle = future.result()

        if not goal_handle.accepted:
            self.get_logger().debug('Goal rejected :(')
            return

        self.get_logger().debug('Goal accepted :)')

        self._get_result_future = goal_handle.get_result_async()

        self._get_result_future.add_done_callback(self.get_result_callback)

        return self._get_result_future

    def get_result_callback(self, future):
        result = future.result().result
        self.get_logger().debug('Result: {0}'.format(result.value))
        shutdown()

    def feedback_callback(self, feedback_msg):
        feedback = feedback_msg.feedback
        self.get_logger().debug('Received feedback: {0}'.format(feedback.partial_sequence))

