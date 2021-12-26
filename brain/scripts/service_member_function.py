#!/usr/bin/env python3

from interfaces.srv import GetStatus, GetStatusKey, SetStatus

from status import Status

import rclpy
import flatdict
from rclpy.node import Node

# TODO: rename this whole thing
class MinimalService(Node):

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

    minimal_service = MinimalService()

    rclpy.spin(minimal_service)

    rclpy.shutdown()


if __name__ == '__main__':
    main()
