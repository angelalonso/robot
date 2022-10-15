#!/usr/bin/env python3

from rclpy import init, logging, shutdown
from rclpy.node import Node

from action_clients import MotorLeftActionClient, MotorRightActionClient, ServoLaserActionClient

from std_msgs.msg import String

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

    def __init__(self, 
            name, 
            loglevel, 
            debugged):
        super().__init__(name)

        self.debugged = debugged 
        self.logger = logging.get_logger(name)
        if ('all' in self.debugged) or ('api' in self.debugged):
            self.logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        #logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        
        # load action clients
        self.motorleft = MotorLeftActionClient()
        self.motorright = MotorRightActionClient()
        self.servolaser = ServoLaserActionClient()
        self.set_status_publisher_ = self.create_publisher(String, 'set_status', 10)
        self.get_status_publisher_ = self.create_publisher(String, 'get_status', 10)
        self.test = -10.0

        self.app = Flask(name)

    def status_listener_callback(self, msg):
        if ('all' in self.debugged) or ('api' in self.debugged):
            self.logger.debug('  - Laser distance: %s' % (msg.data))
        self.radar.add_ping(100, int(msg.data))
        mapping = self.radar.show()
        if ('all' in self.debugged) or ('api' in self.debugged):
            for line in mapping:
                self.logger.debug(line)

    def run(self):
        self.app.run(host="0.0.0.0")

    def add_endpoint(self, endpoint=None, endpoint_name=None, handler=None):
        self.app.add_url_rule(endpoint, endpoint_name, EndpointAction(handler), methods=['POST'])

    def action_stop(self):
        if ('all' in self.debugged) or ('api' in self.debugged):
            self.logger.debug('      STOP')
        future = self.motorleft.send_goal('Stop')
        future = self.motorright.send_goal('Stop')

    def action_fwd(self):
        if ('all' in self.debugged) or ('api' in self.debugged):
            self.logger.debug('      FORWARD')
        future = self.motorleft.send_goal('Forward')
        future = self.motorright.send_goal('Forward')

# we will use this one to test stuff for now
    def action_bwd(self):
        if ('all' in self.debugged) or ('api' in self.debugged):
            self.logger.debug('      BACKWARD')
        future = self.motorleft.send_goal('Backward')
        future = self.motorright.send_goal('Backward')

    def action_right(self):
        if ('all' in self.debugged) or ('api' in self.debugged):
            self.logger.debug('      RIGHT')
        future = self.motorleft.send_goal('Forward')
        future = self.motorright.send_goal('Backward')

    def action_left(self):
        if ('all' in self.debugged) or ('api' in self.debugged):
            self.logger.debug('      LEFT')
        future = self.motorleft.send_goal('Backward')
        future = self.motorright.send_goal('Forward')

    def action_scan(self):
        if ('all' in self.debugged) or ('api' in self.debugged):
            self.logger.debug('      SCAN')

        for i in range(500, 2501, 500):
            if ('all' in self.debugged) or ('api' in self.debugged):
                self.logger.debug('  - rotating: %d' % (i))
            self.servolaser.send_goal(i)
            set_status_msg = String()
            set_status_msg.data = 'servolaser=' + str(i)
            self.set_status_publisher_.publish(set_status_msg)
            get_status_msg = String()
            get_status_msg.data = 'trash'
            self.get_status_publisher_.publish(get_status_msg)
            time.sleep(0.5)
        for i in range(2500, 499, -500):
            if ('all' in self.debugged) or ('api' in self.debugged):
                self.logger.debug('  - rotating: %d' % (i))
            self.servolaser.send_goal(i)
            set_status_msg = String()
            set_status_msg.data = 'servolaser=' + str(i)
            self.set_status_publisher_.publish(set_status_msg)
            get_status_msg = String()
            get_status_msg.data = 'radar'
            self.get_status_publisher_.publish(get_status_msg)
            time.sleep(0.5)
        self.servolaser.send_goal(1500)

    def get_mode(self):
        if ('all' in self.debugged) or ('api' in self.debugged):
            self.logger.debug('  Getting current Mode')
        get_status_msg = String()
        get_status_msg.data = 'mode'
        self.get_status_publisher_.publish(get_status_msg)


def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    DEBUGGED = getenv('DEBUGGED').split(',')

    init(args=args)

    api = ApiWrapper('robotapi', LOGLEVEL, DEBUGGED)
    api.add_endpoint(endpoint='/do/stop', endpoint_name='do_stop', handler=api.action_stop)
    api.add_endpoint(endpoint='/do/fwd', endpoint_name='do_fwd', handler=api.action_fwd)
    api.add_endpoint(endpoint='/do/bwd', endpoint_name='do_bwd', handler=api.action_bwd)
    api.add_endpoint(endpoint='/do/right', endpoint_name='do_right', handler=api.action_right)
    api.add_endpoint(endpoint='/do/left', endpoint_name='do_left', handler=api.action_left)
    api.add_endpoint(endpoint='/do/scan', endpoint_name='do_scan', handler=api.action_scan)
    api.add_endpoint(endpoint='/get/mode', endpoint_name='get_mode', handler=api.get_mode)

    api.run()

    shutdown()

if __name__ == '__main__':
    main()
