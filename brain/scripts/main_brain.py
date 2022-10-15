#!/usr/bin/env python3


from rclpy import init, logging, shutdown
from rclpy.node import Node

from action_clients import LedActionClient, MotorLeftActionClient

from datetime import datetime
from dotenv import load_dotenv
from os import getenv
from time import sleep

class MainNode(Node):

    def __init__(self, 
            loglevel, 
            debugged):
        super().__init__('main')

        self.debugged = debugged 
        #logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        self.logger = logging.get_logger('main')
        if ('all' in self.debugged) or ('api' in self.debugged):
            self.logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))

        self.starttime = datetime.now()

        # load action clients
        self.led= LedActionClient()
        self.test= MotorLeftActionClient()

    def run(self):
        led_state = 0
        while True:

            if led_state == False:
                led_state = True
                if ('all' in self.debugged) or ('main' in self.debugged):
                    self.logger.debug('Turning LED ON')
            else:
                led_state = False
                if ('all' in self.debugged) or ('main' in self.debugged):
                    self.logger.debug('Turning LED OFF')
            self.led.send_goal(led_state)
            sleep(1)

def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    DEBUGGED = getenv('DEBUGGED').split(',')

    init(args=args)

    main_node = MainNode(LOGLEVEL, DEBUGGED)

    main_node.run()

    shutdown() # TODO: needed? maybe for safety when closing?


if __name__ == '__main__':
    main()
