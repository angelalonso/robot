#!/usr/bin/env python3

from rclpy import init, logging, spin, shutdown
from rclpy.node import Node
from rclpy.action import ActionClient

from brain.action import Motor 
from status import Status

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

# --- Goalsets controller

class Goal():
    def __init__(self, parent, parent_repeats, goals_left, launchtime, duration, do):
        self.parent = parent
        self.parent_repeats = parent_repeats
        self.launchtime = launchtime
        self.goals_left = goals_left
        self.duration = duration
        self.do = do
        self.running = False # means it is waiting to run (True means it's running, and when it's done, the goal itself should be deleted)

    def set_running(self):
        self.running = True


class TimedGoals(Node):

    def __init__(self):
        super().__init__('timed_goals')
        self.status = Status()
        self.starttime = datetime.now()
        self.goals = []
        # load set of goals' definitions
        self.load_goalsets()
        # load action clients
        self.motorleft = MotorLeftActionClient()
        self.motorright = MotorRightActionClient()

    def load_goalsets(self):
        self.loaded_goalsets = yaml.load(open('goalsets/main.yaml'))
        for goalset in self.loaded_goalsets:
            goalset['started'] = False # means the goals have NOT yet been loaded to self.goals
            ###self.add_goals(goalset['name'], 0)

    def get_goalset(self, name):
        for actset in self.loaded_goalsets:
            if actset['name'] == name:
                return actset

    def add_goals(self, goalset_name, current):
        goalset = self.get_goalset(goalset_name)
        goals_amount = len(goalset['goals'])
        goal_order = 0
        goal_order_delay = 0
        if goalset['started']:
            goal_start = goalset['start_delay'] + goal_order_delay + current
        else:
            goal_start = goalset['starts_at'] + goalset['start_delay'] + goal_order_delay + current
        for act in goalset['goals']:
            goal_order += 1
            goal = Goal(
                    goalset['name'],
                    goalset['repeat_nr'],
                    goals_amount - goal_order, 
                    goal_start + goal_order_delay,
                    act['time_secs'],
                    act['do']
                    )
            goal_order_delay += act['time_secs']
            self.goals.append(goal)
        if goalset['repeat_nr'] > 0:
            goalset['repeat_nr'] -= 1
        goalset['started'] = True

    def trigger_goalsets(self):
        # this one should substitute trigger_timed_goals
        while True:
            current_raw = datetime.now() - self.starttime
            self.status.set_status('time', current_raw.seconds + (current_raw.microseconds / 1000000))
            # This part handles goalsets
            for goalset in self.loaded_goalsets:
                if goalset['started'] == False:
                    for condition in goalset['conditions_or']:
                        try:
                            if eval(condition):
                                self.add_goals(goalset['name'], self.status['time'])
                                break # we just need one of the conditions to be true
                        except KeyError:
                            self.get_logger().debug('tried checking a variable that does not exist at {}'.format(condition))
                        except NameError:
                            self.get_logger().debug('tried checking a variable that does not exist at {}'.format(condition))

            # This part handles goals
            for go in self.goals:
                # different logic if its already running:
                if go.running:
                    if (go.launchtime + go.duration) <= self.status['time']:
                        self.goals.remove(go)
                        if (go.goals_left == 0 and (go.parent_repeats > 0 or go.parent_repeats == -1)): 
                            self.add_goals(go.parent, self.status['time'])
                else:
                    if go.launchtime <= self.status['time']:
                        self.get_logger().info('doing {} from {} at {}'.format(go.do, go.parent, self.status['time']))
                        go.set_running()
                        self.apply_goal(go.do)

    def apply_goal(self, raw_data):
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

    timed_goals_node = TimedGoals()

    timed_goals_node.trigger_goalsets()

    spin(timed_goals_node)

    timed_goals_node.destroy_node()
    shutdown()


if __name__ == '__main__':
    main()
