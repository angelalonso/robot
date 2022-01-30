#!/usr/bin/env python3

# importing Rust libraries
import importlib.util
rustbrain_spec = importlib.util.spec_from_file_location("rustbrain", 
        "./scripts/rustbrain/.env/lib/python3.8/site-packages/rustbrain/rustbrain.cpython-38-x86_64-linux-gnu.so")
rustbrain = importlib.util.module_from_spec(rustbrain_spec)
rustbrain_spec.loader.exec_module(rustbrain)

from rclpy import init, logging, shutdown, spin_once, ok
from rclpy.node import Node

from action_clients import MotorLeftActionClient, MotorRightActionClient, ServoLaserActionClient
from service_clients import GetStatusKeyServiceClient

import time
from flask import Flask, Response
from dotenv import load_dotenv
from os import getenv
import asyncio

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
        # Temporarily creating this here
        self.radar = rustbrain.Dataset(500, 2500, 500, 10)
        # load action clients
        self.motorleft = MotorLeftActionClient()
        self.motorright = MotorRightActionClient()
        self.servolaser = ServoLaserActionClient()
        self.statuslaser = GetStatusKeyServiceClient()
        self.test = -10.0

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
    def action_bwd(self):
        self.get_logger().info('      BACKWARD')
        future = self.motorleft.send_goal('Backward')
        future = self.motorright.send_goal('Backward')

    def action_right(self):
        self.get_logger().info('      RIGHT')
        future = self.motorleft.send_goal('Forward')
        future = self.motorright.send_goal('Backward')

    def action_left(self):
        self.get_logger().info('      LEFT')
        future = self.motorleft.send_goal('Backward')
        future = self.motorright.send_goal('Forward')

    def action_scan(self):
        self.get_logger().info('      SCAN')
        adddp = rustbrain.Datapoint(6, 4, True)
        self.radar.add(adddp)
        for line in reversed(self.radar.show()):
            self.get_logger().info('| %s |' % line)

        for i in range(500, 2501, 500):
            self.get_logger().info('  - rotating: %d' % (i))
            self.servolaser.send_goal(i)
            self.statuslaser.send_getstatuskey('laser')
            while ok():
                spin_once(self.statuslaser)
                if self.statuslaser.future.done():
                    try:
                        response = self.statuslaser.future.result()
                    except Exception as e:
                        self.get_logger().info('Service call failed %r' % (e,))
                    else:
                        self.get_logger().info('  - Laser distance: %s' % (response.current_status))
                        self.get_logger().info('                        ANGLE \| %d |' % self.radar.add_ping(i, int(response.current_status)))
                    break
            time.sleep(0.5)
        for i in range(2500, 499, -500):
            self.get_logger().info('  - rotating: %d' % (i))
            self.servolaser.send_goal(i)
            self.statuslaser.send_getstatuskey('laser')
            while ok():
                spin_once(self.statuslaser)
                if self.statuslaser.future.done():
                    try:
                        response = self.statuslaser.future.result()
                    except Exception as e:
                        self.get_logger().info('Service call failed %r' % (e,))
                    else:
                        self.get_logger().info('  - Laser distance: %s' % (response.current_status))
                        self.get_logger().info('                        ANGLE \| %d |' % self.radar.add_ping(i, int(response.current_status)))
                    break
            time.sleep(0.5)
        self.servolaser.send_goal(1500)
        adddp = rustbrain.Datapoint(5, 3, True)
        self.radar.add(adddp)
        for line in reversed(self.radar.show()):
            self.get_logger().info('| %s |' % line)


def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')

    init(args=args)

    api = ApiWrapper(LOGLEVEL, 'robotapi')
    api.add_endpoint(endpoint='/do/stop', endpoint_name='do_stop', handler=api.action_stop)
    api.add_endpoint(endpoint='/do/fwd', endpoint_name='do_fwd', handler=api.action_fwd)
    api.add_endpoint(endpoint='/do/bwd', endpoint_name='do_bwd', handler=api.action_bwd)
    api.add_endpoint(endpoint='/do/right', endpoint_name='do_right', handler=api.action_right)
    api.add_endpoint(endpoint='/do/left', endpoint_name='do_left', handler=api.action_left)
    api.add_endpoint(endpoint='/do/scan', endpoint_name='do_scan', handler=api.action_scan)

    api.run()

    shutdown()

if __name__ == '__main__':
    main()
