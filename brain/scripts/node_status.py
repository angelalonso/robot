#!/usr/bin/env python3

# importing Rust libraries
import importlib.util

built_lib = "./scripts/robotlogic/robotlogic.so"
robotlogic_spec = importlib.util.spec_from_file_location("robotlogic", 
    built_lib)
robotlogic = importlib.util.module_from_spec(robotlogic_spec)
robotlogic_spec.loader.exec_module(robotlogic)

from rclpy import init, logging, spin, shutdown
from rclpy.node import Node

from action_clients import LedActionClient, MotorLeftActionClient, MotorRightActionClient

from std_msgs.msg import String

#from datetime import datetime as dt
from dotenv import load_dotenv
from os import getenv
from os.path import exists
from sys import exit
import flatdict
import time

class Status(object):
    def __init__(self):
        super().__init__()
        self.current = {}
        

    def __getitem__(self, item):
        try:
            return self.current[item]
        except KeyError:
            raise

    def set_status(self, element, value):
        self.current[element] = value

    def get_status(self):
        return str(flatdict.FlatDict(self.current, delimiter='.'))


class StatusManager(Node):

    def __init__(self, 
            name, 
            loglevel, 
            debugged, 
            mode, 
            refresh_secs):
        super().__init__(name)

        self.debugged = debugged 
        self.logger = logging.get_logger(name)
        if ('all' in self.debugged or 
                'status_logic' in self.debugged or 
                'status_logic_led' in self.debugged or 
                'status_logic_motors' in self.debugged or 
                'status_logic_motor_l' in self.debugged or 
                'status_logic_motor_r' in self.debugged or 
                'status' in self.debugged):
            self.logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        #logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))

        self.status = Status()
        self.status.set_status("mode", mode)

        self.start = time.time()

        # TODO: continue this:
        if mode == "callibrating":
            actionset_file = "callibration_actionset.yaml"
        else:
            actionset_file = "integration_actionset.yaml"
        if not exists(actionset_file):
            self.get_logger().error("File " + actionset_file + " does not exist")
            exit("File " + actionset_file + " does not exist")
        self.logic = robotlogic.Logic(mode, actionset_file, 500, 2500, 500, 10)
        self.logic.radar_init()
        # Listen to `get_status` (to send status after that)
        self.getstatus_subscription = self.create_subscription(
            String,
            'get_status',
            self.listener_callback_get,
            10)

        # load action clients
        self.led= LedActionClient()
        self.motor_l= MotorLeftActionClient()
        self.motor_r= MotorRightActionClient()

        self.getstatus_subscription  # prevent unused variable warning        
        # Listen to `set_status` (to change the value of a status after that)
        self.setstatus_subscription = self.create_subscription(
            String,
            'set_status',
            self.listener_callback_set,
            10)
        try:
            rate = float(refresh_secs)
        except:
            self.get_logger().warn('REFRESH_SECS env variable is not defined as a proper float value')
            rate = 2 # too high a default value, to make it noticeable that we re using the default
        self.timer = self.create_timer(rate, self.logic_callback)
        self.setstatus_subscription  # prevent unused variable warning        

    def logic_callback(self):
        now = time.time()
        self.logic.set_state("time", str(now - self.start))
        result = self.logic.get_action()
        if ('all' in self.debugged or 
                'status_logic' in self.debugged or 
                'status_logic_led' in self.debugged or 
                'status_logic_motors' in self.debugged or 
                'status_logic_motor_l' in self.debugged or 
                'status_logic_motor_r' in self.debugged or 
                'status' in self.debugged):
            self.logger.debug('-- Logic says: ' + result)
        for action in result.split(","):
            # TODO: clean up spaces before and after that comma
            try:
                key, value = action.split("=")
                # TODO: clean up spaces before and after that equal sign
            except ValueError:
                if ('all' in self.debugged or 
                        'status_logic' in self.debugged or 
                        'status_logic_led' in self.debugged or 
                        'status_logic_motors' in self.debugged or 
                        'status_logic_motor_l' in self.debugged or 
                        'status_logic_motor_r' in self.debugged or 
                        'status' in self.debugged):
                    self.logger.debug('ERROR Splitting ---- ' + action)
            else:
                try:
                    #self.logic.add_object(key, int(value)) # TODO: do we need this? then solve issue
                    if (key == "led"):
                        self.led.send_goal(value.lower() in ['true', '1', 't', 'y', 'yes'])
                        if ('all' in self.debugged or 
                                'status' in self.debugged or 
                                'status_logic_led' in self.debugged):
                            self.logger.debug('---- ' + key + ' -> ' + value)
                        self.status.set_status(key, value.lower() in ['true', '1', 't', 'y', 'yes'])
                    elif (key == "motor_l"):
                        self.motor_l.send_goal(value)
                        if ('all' in self.debugged or 
                                'status' in self.debugged or 
                                'status_logic_motors' in self.debugged or 
                                'status_logic_motor_l' in self.debugged):
                            self.logger.debug('---- ' + key + ' -> ' + value)
                        self.status.set_status(key, value)
                    elif (key == "motor_r"):
                        self.motor_r.send_goal(value)
                        if ('all' in self.debugged or 
                                'status' in self.debugged or 
                                'status_logic_motors' in self.debugged or 
                                'status_logic_motor_r' in self.debugged):
                            self.logger.debug('---- ' + key + ' -> ' + value)
                        self.status.set_status(key, value)
                    else:
                        pass
                except Exception as e:
                    if ('all' in self.debugged) or ('status' in self.debugged):
                        self.logger.debug('-- PROBLEM SPLITTING ' + result + '--' + str(e))
        #self.logic.do_next_action()
        #try:
        #    login_return_msg = self.logic.get_state("logic_log_msg")
        #except KeyError:
        #    login_return_msg = ""
        #if login_return_msg != "":
        #    self.logger.debug('-- Logic says: ' + login_return_msg)
        #    self.logic.set_empty_state("logic_log_msg")

    def listener_callback_set(self, msg):
        # TODO: improve on this (avoid using SENSOR if not necessary?)
        data_stripped = msg.data.replace("SENSOR: ", "").split("|")
        for keyval_raw in data_stripped:
            keyval = keyval_raw.split("=")
            try:
                self.status.set_status(keyval[0], keyval[1])
                if keyval[0] == 'laser':
                    try:
                        self.logic.add_object(int(self.status['servolaser']), int(keyval[1]))
                    except (ValueError, KeyError):
                        pass
                if ('all' in self.debugged or 
                        'status_no_logic' in self.debugged or
                        'status' in self.debugged):
                    self.logger.debug('I heard SET: ' + keyval[0] + ' to ' + keyval[1])
            except IndexError:
                if ('all' in self.debugged or 
                        'status' in self.debugged):
                    self.logger.debug('ERROR SETTING: ' + keyval_raw + '.')

    def listener_callback_get(self, msg):
        if ('all' in self.debugged or 
                'status' in self.debugged):
            self.logger.debug('I heard GET: "%s"' % msg.data)
        if msg.data == 'radar':
            mapping = self.logic.get_radar()
            for line in mapping:
                if ('all' in self.debugged or 
                        'status' in self.debugged):
                    self.logger.debug(line)
        else:
            try:
                if ('all' in self.debugged or 
                        'status' in self.debugged):
                    self.logger.debug(self.status[msg.data])
            except KeyError:
                if ('all' in self.debugged or 
                        'status' in self.debugged):
                    self.logger.debug("NOOOOOOOOOOOOOONE")



def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    raw_debugged = getenv('DEBUGGED')
    try:
        DEBUGGED = str(raw_debugged).split(',')
    except:
        DEBUGGED = []
    MODE = getenv('MODE')
    REFRESH_SECS = getenv('REFRESH_SECS')

    init(args=args)

    status_manager = StatusManager('status_proxy', LOGLEVEL, DEBUGGED, MODE, REFRESH_SECS)

    spin(status_manager)

    status_manager.destroy_node()
    shutdown()


if __name__ == '__main__':
    main()

