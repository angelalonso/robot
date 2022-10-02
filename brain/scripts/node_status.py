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

from std_msgs.msg import String

from datetime import datetime as dt
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

    def __init__(self, loglevel, mode, refresh_secs):
        super().__init__('status_publisher')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
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
        # TODO: logic has a specific function to check what to do on this specific call, by checking statuses, mainly time 
        now = time.time()
        self.logic.set_state("time", dt.strftime(dt.fromtimestamp(now - self.start - 3600), '%H:%M:%S.%f')) # TODO: why do I need to remove an hour here???
        result = self.logic.get_action()
        self.get_logger().info('-- Logic says: ' + result)

        #self.logic.do_next_action()
        #try:
        #    login_return_msg = self.logic.get_state("logic_log_msg")
        #except KeyError:
        #    login_return_msg = ""
        #if login_return_msg != "":
        #    self.get_logger().info('-- Logic says: ' + login_return_msg)
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
                self.get_logger().debug('I heard SET: ' + keyval[0] + ' to ' + keyval[1])
            except IndexError:
                self.get_logger().debug('ERROR SETTING: ' + keyval_raw + '.')

    def listener_callback_get(self, msg):
        self.get_logger().info('I heard GET: "%s"' % msg.data)
        if msg.data == 'radar':
            ##mapping = self.radar.show()
            mapping = self.logic.get_radar()
            ##for line in mapping:
            ##    self.get_logger().info(line)
            for line in mapping:
                self.get_logger().info(line)
        else:
            try:
                self.get_logger().info(self.status[msg.data])
            except KeyError:
                self.get_logger().info("NOOOOOOOOOOOOOONE")



def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    MODE = getenv('MODE')
    REFRESH_SECS = getenv('REFRESH_SECS')

    init(args=args)

    status_manager = StatusManager(LOGLEVEL, MODE, REFRESH_SECS)

    spin(status_manager)

    status_manager.destroy_node()
    shutdown()


if __name__ == '__main__':
    main()

