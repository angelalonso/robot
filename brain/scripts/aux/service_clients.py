#!/usr/bin/env python3

from rclpy import ok, spin_once
from rclpy.node import Node

from interfaces.srv import GetStatusKey

class GetStatusKeyServiceClient(Node):

    def __init__(self):
        super().__init__('get_status_key_action_client')
        self.getstatuskey_cli = self.create_client(GetStatusKey, 'getstatuskey')
        while not self.getstatuskey_cli.wait_for_service(timeout_sec=1.0):
            self.get_logger().debug('service not available, waiting again...')
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

