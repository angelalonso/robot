#!/usr/bin/env python3

from rclpy import init, logging, shutdown
from rclpy.node import Node

from action_clients import MotorLeftActionClient, MotorRightActionClient, ServoLaserActionClient

from flask import Flask, Response
from dotenv import load_dotenv
from os import getenv

class EndpointAction(object):

    def __init__(self, action):
        self.action = action
        self.response = Response(status=200, headers={})

    def __call__(self, *args):
        self.action()
        return self.response


class ApiWrapper(Node):
    app = None

    def __init__(self, loglevel, name):
        super().__init__('api')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        # load action clients
        self.motorleft = MotorLeftActionClient()
        self.motorright = MotorRightActionClient()
        self.servolaser = ServoLaserActionClient()
        self.test = 20.0

        self.app = Flask(name)

    def run(self):
        self.app.run(host="0.0.0.0")

    def add_endpoint(self, endpoint=None, endpoint_name=None, handler=None):
        self.app.add_url_rule(endpoint, endpoint_name, EndpointAction(handler), methods=['POST'])

    def action_stop(self):
        self.get_logger().info('      STOP')
        future = self.motorleft.send_goal('Stop')
        future = self.motorright.send_goal('Stop')

    def action_fwd(self):
        self.get_logger().info('      FORWARD')
        future = self.motorleft.send_goal('Forward')
        future = self.motorright.send_goal('Forward')

# we will use this one to test stuff for now
    def action_back(self):
        #self.get_logger().info('      BACKWARD')
        #future = self.motorleft.send_goal('Backward')
        #future = self.motorright.send_goal('Backward')
        self.get_logger().info('      TESTING')
        #TODO: add Codes for actions on higher numbers, create some IFs at the other side
        self.test = (30 + (self.test - 20.0)) % 100
        future = self.servolaser.send_goal(self.test)

    def action_right(self):
        self.get_logger().info('      RIGHT')
        future = self.motorleft.send_goal('Forward')
        future = self.motorright.send_goal('Backward')

    def action_left(self):
        self.get_logger().info('      LEFT')
        future = self.motorleft.send_goal('Backward')
        future = self.motorright.send_goal('Forward')

def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')

    init(args=args)

    api = ApiWrapper(LOGLEVEL, 'robotapi')
    api.add_endpoint(endpoint='/do/motor_l=0,motor_r=0', endpoint_name='do_stop', handler=api.action_stop)
    api.add_endpoint(endpoint='/do/motor_l=100,motor_r=100', endpoint_name='do_fwd', handler=api.action_fwd)
    api.add_endpoint(endpoint='/do/motor_l=-100,motor_r=-100', endpoint_name='do_back', handler=api.action_back)
    api.add_endpoint(endpoint='/do/motor_l=-100,motor_r=100', endpoint_name='do_right', handler=api.action_right)
    api.add_endpoint(endpoint='/do/motor_l=100,motor_r=-100', endpoint_name='do_left', handler=api.action_left)

    api.run()

    shutdown()

if __name__ == '__main__':
    main()
