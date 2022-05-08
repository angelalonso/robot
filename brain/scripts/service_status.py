#!/usr/bin/env python3

from interfaces.srv import GetStatus, GetStatusKey, SetStatus

from rclpy import init, logging, spin, shutdown
import flatdict
from rclpy.node import Node

from brain.action import Getstatus, Whatstatus

from dotenv import load_dotenv
from os import getenv


class Status(object):
    def __init__(self):
        super().__init__()
        self.current = {}

    def __getitem__(self, item):
         return self.current[item]

    def set_status(self, element, value):
        self.current[element] = value

    def get_status(self):
        # TODO: this is NOT the bottleneck
        return str(flatdict.FlatDict(self.current, delimiter='.'))

class StatusActionServer(Node):
    def __init__(self, loglevel):
        super().__init__('status_action_server')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        self.status = Status()

        self._action_server = ActionServer(
            self,
            Getstatus,
            'GetStatusServo',
            self.execute_callback)

    def execute_callback(self, goal_handle):
        feedback_msg = Motor.Feedback()
        goal_handle.publish_feedback(feedback_msg)

        goal_handle.request.key

        goal_handle.succeed()
        result = Getstatus.Result()
        return result


class StatusService(Node):

    def __init__(self, loglevel):
        super().__init__('minimal_service')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        self.status = Status()
        self.getstatus_srv = self.create_service(GetStatus, 'getstatus', self.getstatus_callback)
        self.getstatuskey_srv = self.create_service(GetStatusKey, 'getstatuskey', self.getstatuskey_callback)
        # TODO: this is the bottleneck
        self.setstatus_srv = self.create_service(SetStatus, 'setstatus', self.setstatus_callback)

    def getstatus_callback(self, request, response):
        self.get_logger().debug('Incoming request to get status')
        response.current_status = self.status.get_status()
        return response

    def getstatuskey_callback(self, request, response):
        self.get_logger().info('Incoming request - key: %s' % (request.key))
        try:
            response.current_status = self.status[request.key]
        except KeyError:
            response.current_status = ''
        return response

    def setstatus_callback(self, request, response):
        self.get_logger().debug('Incoming request - key: %s value: %s' % (request.key, request.value))
        self.status.set_status(request.key, request.value)
        # TODO: this is the bottleneck - 2
        response.current_status = self.status.get_status()
        return response


def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')

    init(args=args)

    service_status = StatusService(LOGLEVEL)

    spin(service_status)

    shutdown()

if __name__ == '__main__':
    main()
