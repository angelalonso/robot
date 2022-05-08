#!/usr/bin/env python3

from rclpy import init, logging, shutdown, spin_once, ok
from rclpy.node import Node

from dotenv import load_dotenv
from os import getenv


# TODO:
# Start callibrate program
#   Load callibration plan
#   Take actions, based on time
#   Receive metric
#   Check if new loop is needed
class Callibrate(Node):
    def __init__(self, loglevel, name):
        super().__init__(name)
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        self.running = False

    def run(self):
        self.running = True
        while self.running:
            self.running = False
        self.get_logger().error('---------------------------- DONE RUNNING')



def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')

    init(args=args)

    callibrate = Callibrate(LOGLEVEL, 'callibrate')

    callibrate.run()

    shutdown()

if __name__ == '__main__':
    main()
