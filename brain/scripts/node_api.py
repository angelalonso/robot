#!/usr/bin/env python3

from rclpy import init, logging, shutdown
from rclpy.node import Node

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
        self.app = Flask(name)

    def run(self):
        self.app.run(host="0.0.0.0")

    def add_endpoint(self, endpoint=None, endpoint_name=None, handler=None):
        self.app.add_url_rule(endpoint, endpoint_name, EndpointAction(handler))


# TODO: 
#[node_api.py-2] 192.168.1.2 - - [29/Dec/2021 16:22:19] "POST /do/motor_l=100,motor_r=100 HTTP/1.1" 404 -
#[main_brain.py-1] [INFO] [1640791340.460078960] [brain.main_brain]: doing motorleft=Forward|motorright=Backward from stop_n_back_when_too_close at 10.740808
#[node_motor_left_worker.py-7] [INFO] [1640791340.461802892] [brain.node_motor_left_worker]: Feedback: moving Left Motor Forward
#[node_api.py-2] 192.168.1.2 - - [29/Dec/2021 16:22:20] "POST /do/motor_l=0,motor_r=0 HTTP/1.1" 404 -
#[main_brain.py-1] [INFO] [1640791340.999987046] [brain.main_brain]: doing motorleft=Forward|motorright=Forward from move_when_free at 11.280517
#[node_motor_right_worker.py-6] [INFO] [1640791341.001800740] [brain.node_motor_right_worker]: Feedback: moving Right Motor Forward
#[node_api.py-2] 192.168.1.2 - - [29/Dec/2021 16:22:23] "POST /do/motor_l=100,motor_r=-100 HTTP/1.1" 404 -
#[node_api.py-2] 192.168.1.2 - - [29/Dec/2021 16:22:24] "POST /do/motor_l=0,motor_r=0 HTTP/1.1" 404 -
#[node_api.py-2] 192.168.1.2 - - [29/Dec/2021 16:22:27] "POST /do/motor_l=-100,motor_r=100 HTTP/1.1" 404 -
#[node_api.py-2] 192.168.1.2 - - [29/Dec/2021 16:22:27] "POST /do/motor_l=0,motor_r=0 HTTP/1.1" 404 -
#[node_api.py-2] 192.168.1.2 - - [29/Dec/2021 16:22:32] "POST /do/motor_l=-100,motor_r=-100 HTTP/1.1" 404 -
#[node_api.py-2] 192.168.1.2 - - [29/Dec/2021 16:22:33] "POST /do/motor_l=0,motor_r=0 HTTP/1.1" 404 -

    def action(self):
        self.get_logger().info('service not available, waiting again...')


def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')

    init(args=args)

    api = ApiWrapper(LOGLEVEL, 'robotapi')
    api.add_endpoint(endpoint='/ad', endpoint_name='ad', handler=api.action)

    api.run()

    shutdown()

if __name__ == '__main__':
    main()
