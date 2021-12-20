#!/usr/bin/env python3

from rclpy import init, logging, spin, shutdown
from rclpy.node import Node

class TimedActions(Node):

    def __init__(self):
        super().__init__('timed_actions')
        self.get_logger().info('OK ')
        # load set of actions' definitions
        # loop that applies actions when:
        # - time is equal of bigger than defined
        # - previous action-set is finished AND it is set to repeat (more than 0 times, or forever)

def main(args=None):
    init(args=args)

    timed_actions_node = TimedActions()

    spin(timed_actions_node)

    timed_actions_node.destroy_node()
    shutdown()


if __name__ == '__main__':
    main()
