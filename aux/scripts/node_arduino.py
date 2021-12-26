#!/usr/bin/env python3

from rclpy import init
from rclpy import shutdown
from rclpy import logging
from rclpy.node import Node
from processing import process_input
from processing import get_test
from processing import info_entries
from std_msgs.msg import String

from dotenv import load_dotenv
from os import getenv
from random import randint
from random import seed
import hashlib
import time

def mock_distance():
    result = randint(1, 999)
    return result

class SerialLink(Node):

    def __init__(self, loglevel):
        super().__init__('arduino_serial_sync_mocked')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        self.publisher_ = self.create_publisher(String, 'main_topic', 10)
        self.latest_infos = info_entries()
        self.mocked_list = [] 
        self.mocked_list_ix = 0
        filename = './arduino_mocked_values.list'
        with open(filename) as file:
            for line in file:
                self.mocked_list.append(line.strip())

    def get_value(self):
        try:
            value = self.mocked_list[self.mocked_list_ix]
        except IndexError:
            self.mocked_list_ix = 0
            value = self.mocked_list[self.mocked_list_ix]
        self.mocked_list_ix += 1
        result = "SENSOR: distance=" + value + "|"
        self.get_logger().info(result)
        return result

    def sync_and_read(self, read_delay):
        while True:
            msg = String()
            msg.data = process_input(self.latest_infos, self.get_value())
            self.publisher_.publish(msg)
            time.sleep(read_delay)


def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    ARDUINO_READ_DELAY = float(getenv('ARDUINO_READ_DELAY'))

    init(args=args)

    arduino_serial = SerialLink(LOGLEVEL)

    arduino_serial.sync_and_read(ARDUINO_READ_DELAY)

    shutdown()

if __name__ == '__main__':
    main()