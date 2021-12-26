#!/usr/bin/env python3

from interfaces.srv import GetStatus, GetStatusKey, SetStatus

import rclpy
import flatdict
from rclpy.node import Node

class Status(object):
    def __init__(self):
        super().__init__()
        self.current = {}

    def __getitem__(self, item):
         return self.current[item]

    def set_status(self, element, value):
        self.current[element] = value

    def get_status(self):
        return str(flatdict.FlatDict(self.current, delimiter='.'))

class StatusService(Node):

    def __init__(self):
        super().__init__('minimal_service')
        self.status = Status()
        self.getstatus_srv = self.create_service(GetStatus, 'getstatus', self.getstatus_callback)
        self.getstatuskey_srv = self.create_service(GetStatusKey, 'getstatuskey', self.getstatuskey_callback)
        self.setstatus_srv = self.create_service(SetStatus, 'setstatus', self.setstatus_callback)

    def getstatus_callback(self, request, response):
        self.get_logger().debug('Incoming request to get status')
        response.current_status = self.status.get_status()
        return response

    def getstatuskey_callback(self, request, response):
        self.get_logger().debug('Incoming request - key: %s' % (request.key))
        try:
            response.current_status = self.status[request.key]
        except KeyError:
            pass
        return response

    def setstatus_callback(self, request, response):
        self.get_logger().debug('Incoming request - key: %s value: %s' % (request.key, request.value))
        self.status.set_status(request.key, request.value)
        response.current_status = self.status.get_status()
        return response


def main(args=None):
    rclpy.init(args=args)

    service_status = StatusService()

    rclpy.spin(service_status)

    rclpy.shutdown()


if __name__ == '__main__':
    main()
