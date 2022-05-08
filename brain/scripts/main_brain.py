#!/usr/bin/env python3


from rclpy import init, logging, spin, spin_once, shutdown, ok
from rclpy.node import Node

from interfaces.srv import GetStatus, GetStatusKey, SetStatus
from service_status import Status
from action_clients import MotorLeftActionClient, MotorRightActionClient
from service_clients import GetStatusKeyServiceClient

from datetime import datetime
from dotenv import load_dotenv
from os import getenv
from std_msgs.msg import String
import yaml
import time

# TODO: not needed?

#class Goal():
#    def __init__(self, parent, parent_repeats, goals_left, launchtime, duration, do):
#        self.parent = parent
#        self.parent_repeats = parent_repeats
#        self.launchtime = launchtime
#        self.goals_left = goals_left
#        self.duration = duration
#        self.do = do
#        self.running = False # means it is waiting to run (True means it's running, and when it's done, the goal itself should be deleted)
#
#    def set_running(self):
#        self.running = True


class TimedGoals(Node):

    def __init__(self, loglevel):
        super().__init__('timed_goals')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))

        ##self.setstatus_cli = self.create_client(SetStatus, 'setstatus')
        ##while not self.setstatus_cli.wait_for_service(timeout_sec=1.0):
        ##    self.get_logger().info('service not available, waiting again...')
        ##self.setstatus_req = SetStatus.Request()

        #self.getstatuskey_cli = self.create_client(GetStatusKey, 'getstatuskey')
        #while not self.getstatuskey_cli.wait_for_service(timeout_sec=1.0):
        #    self.get_logger().info('service not available, waiting again...')
        #self.getstatuskey_req = GetStatusKey.Request()
        self.getstatuskey_cli = GetStatusKeyServiceClient()

        self.starttime = datetime.now()
        self.goals = []
        # load set of goals' definitions
        self.load_goalsets()
        # load action clients
        self.motorleft = MotorLeftActionClient()
        self.motorright = MotorRightActionClient()

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

    ##def send_setstatus_req(self, key, value):
    ##    self.setstatus_req.key = key
    ##    self.setstatus_req.value = value 
    ##    self.future = self.setstatus_cli.call_async(self.setstatus_req)

    ##def send_setstatus(self, key, value):
    ##    result = ""
    ##    self.send_setstatus_req(key, str(value))
    ##    while ok():
    ##        spin_once(self)
    ##        if self.future.done():
    ##            try:
    ##                response = self.future.result()
    ##            except Exception as e:
    ##                self.get_logger().debug('Service call failed %r' % (e,))
    ##            else:
    ##                result = response.current_status
    ##            break
    ##    return result

    def load_goalsets(self):
        self.loaded_goalsets = yaml.load(open('goalsets/main.yaml'))
        try:
            for goalset in self.loaded_goalsets:
                goalset['started'] = False # means the goals have NOT yet been loaded to self.goals
                ###self.add_goals(goalset['name'], 0)
        except TypeError:
            self.get_logger().debug('No Goalsets available')

    def get_goalset(self, name):
        for actset in self.loaded_goalsets:
            if actset['name'] == name:
                return actset

    def set_goalset_active(self, name, curr_time):
        # When one goalset goes active, the others go inactive
        self.clear_goals()
        self.add_goals(name, curr_time)
        for actset in self.loaded_goalsets:
            if actset['name'] == name:
                actset['started'] = True
            else:
                actset['started'] = False

    def set_goalset_done(self, name):
        for actset in self.loaded_goalsets:
            if actset['name'] == name:
                actset['started'] = False

    def clear_goals(self):
        for ix in range(len(self.goals)):
            self.goals.remove(self.goals[0])

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

    def trigger_goalsets(self):
        while True:
            current_raw = datetime.now() - self.starttime
            curr_time = current_raw.seconds + (current_raw.microseconds / 1000000)
            ##self.send_setstatus('time', curr_time)
            # This part handles goalsets
            try:
                for goalset in self.loaded_goalsets:
                    if goalset['started'] == False:
                        for condition in goalset['conditions_or']:
                            try:
                                if eval(condition):
                                    self.set_goalset_active(goalset['name'], curr_time)
                                    break # we just need one of the conditions to be true
                            except (ValueError, KeyError, NameError):
                                self.get_logger().debug('tried checking a variable that does not exist at {}'.format(condition))
            except TypeError:
                self.get_logger().debug('No Goalsets available')
            # This part handles goals
            for go in self.goals:
                # different logic if its already running:
                if go.running:
                    if go.duration != -1:
                        if (go.launchtime + go.duration) <= curr_time:
                            self.goals.remove(go)
                            if (go.goals_left == 0 and (go.parent_repeats > 0 or go.parent_repeats == -1)): 
                                self.add_goals(go.parent, curr_time)
                            if go.goals_left == 0:
                                if (go.parent_repeats > 0 or go.parent_repeats == -1): 
                                    self.add_goals(go.parent, curr_time)
                                else:
                                    self.set_goalset_done(go.parent)
                else:
                    if go.launchtime <= curr_time:
                        self.get_logger().info('doing {} from {} at {}'.format(go.do, go.parent, curr_time))
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
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')

    init(args=args)

    timed_goals_node = TimedGoals(LOGLEVEL)

    timed_goals_node.trigger_goalsets()

    spin(timed_goals_node)

    timed_goals_node.destroy_node()
    shutdown()


if __name__ == '__main__':
    main()
