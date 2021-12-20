#!/usr/bin/env python3

from rclpy import init, logging, spin, shutdown
from rclpy.node import Node

from datetime import datetime
import yaml

class TimedActions(Node):

    def __init__(self):
        super().__init__('timed_actions')
        self.starttime = datetime.now()
        # load set of actions' definitions
        self.load_actionsets()
        # loop that applies actions when:
        # - time is equal of bigger than defined
        # - previous action-set is finished AND it is set to repeat (more than 0 times, or forever)

    def load_actionsets(self):
        self.actionsets = yaml.load(open('actionsets/actionset.yaml'))
        #self.get_logger().info('%s' % self.actionsets[0]['name'])

    def trigger_actions(self):
        # todo: distinguish between triggered and untriggered ones
        while True:
            current_raw = datetime.now() - self.starttime
            current = current_raw.seconds + (current_raw.microseconds / 1000000)
            for action in self.actionsets:
                if (action['starts_at'] + action['starts_after']) <= current:
                    self.get_logger().info('%s' % current)
                    self.get_logger().info('%s' % action['actions'][0])




def main(args=None):
    init(args=args)

    timed_actions_node = TimedActions()

    timed_actions_node.trigger_actions()

    shutdown()


if __name__ == '__main__':
    main()
