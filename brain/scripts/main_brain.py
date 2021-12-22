#!/usr/bin/env python3

from rclpy import init, logging, spin, shutdown
from rclpy.node import Node
from rclpy.action import ActionClient

from brain.action import Motor 

from datetime import datetime
import yaml

# --- Action Clients

class MotorLeftActionClient(Node):

    def __init__(self):
        super().__init__('motor_left_action_client')
        self._action_client = ActionClient(self, Motor, 'MotorLeft')

    def send_goal(self, move):
        goal_msg = Motor.Goal()
        goal_msg.move = move

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)

class MotorRightActionClient(Node):

    def __init__(self):
        super().__init__('motor_right_action_client')
        self._action_client = ActionClient(self, Motor, 'MotorRight')

    def send_goal(self, move):
        goal_msg = Motor.Goal()
        goal_msg.move = move

        self._action_client.wait_for_server()

        return self._action_client.send_goal_async(goal_msg)

# --- Actionsets controller

class Action():
    def __init__(self, parent, parent_repeats, actions_left, launchtime, duration, do):
        self.parent = parent
        self.parent_repeats = parent_repeats
        self.launchtime = launchtime
        self.actions_left = actions_left
        self.duration = duration
        self.do = do
        self.running = False # means it is waiting to run (True means it's running, and when it's done, the action itself should be deleted)

    def set_running(self):
        self.running = True


class TimedActions(Node):

    def __init__(self):
        super().__init__('timed_actions')
        self.starttime = datetime.now()
        self.actions = []
        # load set of actions' definitions
        self.load_actionsets()
        # load action clients
        self.motorleft = MotorLeftActionClient()
        self.motorright = MotorRightActionClient()

    def load_actionsets(self):
        self.loaded_actionsets = yaml.load(open('actionsets/actionset.yaml'))
        for actionset in self.loaded_actionsets:
            actionset['started'] = False # means the actions have NOT yet been loaded to self.actions
            self.add_actions(actionset['name'], 0)

    def get_actionset(self, name):
        for actset in self.loaded_actionsets:
            if actset['name'] == name:
                return actset

    def add_actions(self, actionset_name, current):
        actionset = self.get_actionset(actionset_name)
        actions_amount = len(actionset['actions'])
        action_order = 0
        action_order_delay = 0
        if actionset['started']:
            action_start = actionset['start_delay'] + action_order_delay + current
        else:
            action_start = actionset['starts_at'] + actionset['start_delay'] + action_order_delay + current
        for act in actionset['actions']:
            action_order += 1
            action = Action(
                    actionset['name'],
                    actionset['repeat_nr'],
                    actions_amount - action_order, 
                    action_start + action_order_delay,
                    act['time_secs'],
                    act['do']
                    )
            action_order_delay += act['time_secs']
            self.actions.append(action)
        if actionset['repeat_nr'] > 0:
            actionset['repeat_nr'] -= 1
        actionset['started'] = True

    def trigger_timed_actions(self):
        # todo: distinguish between triggered and untriggered ones
        while True:
            current_raw = datetime.now() - self.starttime
            current = current_raw.seconds + (current_raw.microseconds / 1000000)
            for act in self.actions:
                if act.running:
                    if (act.launchtime + act.duration) <= current:
                        self.actions.remove(act)
                        if (act.actions_left == 0 and (act.parent_repeats > 0 or act.parent_repeats == -1)): 
                            self.add_actions(act.parent, current)

                else:
                    if act.launchtime <= current:
                        self.get_logger().info('doing {} from {} at {}'.format(act.do, act.parent, current))
                        act.set_running()
                        self.apply_action(act.do)

    def apply_action(self, raw_data):
        descriptions = raw_data.split('|')
        for description_raw in descriptions:
            description = description_raw.split('=')
            if description[0] == 'motorleft':
                future = self.motorleft.send_goal(description[1])
                #spin_until_future_complete(self.motorleft, future) # Needed??
            elif description[0] == 'motorright':
                future = self.motorright.send_goal(description[1])






def main(args=None):
    init(args=args)

    timed_actions_node = TimedActions()

    timed_actions_node.trigger_timed_actions()

    spin(timed_actions_node)

    timed_actions_node.destroy_node()
    shutdown()


if __name__ == '__main__':
    main()
