#!/usr/bin/env python3

from rclpy import init, logging, spin, shutdown
from rclpy.node import Node
from rclpy.action import ActionClient

from dotenv import load_dotenv
from os import getenv
from std_msgs.msg import String
import flatdict

# we probably want to remove or merge this with the status service
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

class TopicStatusSubscriber(Node):
    def __init__(self, loglevel):
        super().__init__('topic_status_subscriber')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        self.subscription = self.create_subscription(
            String,
            'topic_status',
            self.listener_callback,
            10)
        self.subscription  # prevent unused variable warning

    def listener_callback(self, msg):
        if msg.data[0:8] == 'SENSOR: ':
            sensors = msg.data.replace('\n', '').replace('SENSOR: ', '').split('|')
            for sensor_raw in sensors:
                if sensor_raw != '':
                    sensor = sensor_raw.split('=')
                    try:
                        self.main_status.set_status(sensor[0], sensor[1])
                    except IndexError: pass
                    # TODO: if it's a SENSORS message, update Status

def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')

    init(args=args)
    topic_status_node = TopicStatusSubscriber(LOGLEVEL)

    spin(topic_status_node)

    topic_status_node.destroy_node()
    shutdown()


if __name__ == '__main__':
    main()
