#!/usr/bin/env python3

from rclpy import init, logging, shutdown, spin
from rclpy.action import ActionServer
from rclpy.node import Node
try:
    import RPi.GPIO as GPIO
except ModuleNotFoundError:
    from lib_fake_rpi import fake_rpi as GPIO

from brain.action import Led

from dotenv import load_dotenv
from os import getenv

class LEDMainActionServer(Node):

    def __init__(self, 
            name, 
            loglevel, 
            debugged, 
            PinNr):
        super().__init__(name)

        self.debugged = debugged
        self.logger = logging.get_logger(name)
        if ('all' in self.debugged) or ('led' in self.debugged):
            self.logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        #logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))

        self.pin_id = PinNr
        self.state = False
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(self.pin_id, GPIO.OUT)

        self._action_server = ActionServer(
            self,
            Led,
            'LedMain',
            self.execute_callback)

    def execute_callback(self, goal_handle):
        feedback_msg = Led.Feedback()
        goal_handle.publish_feedback(feedback_msg)

        if goal_handle.request.turn_on == True:
            feedback_msg.process_feed = "setting PIN " + str(self.pin_id) + " On"
            if ('all' in self.debugged) or ('led' in self.debugged):
                self.logger.debug("setting PIN " + str(self.pin_id) + " On")
            GPIO.output(self.pin_id, GPIO.HIGH)
            self.turn_on = True
        else:
            feedback_msg.process_feed = "setting PIN " + str(self.pin_id) + " Off"
            if ('all' in self.debugged) or ('led' in self.debugged):
                self.logger.debug("setting PIN " + str(self.pin_id) + " Off")
            GPIO.output(self.pin_id, GPIO.LOW)
            self.turn_on = False

        if ('all' in self.debugged) or ('led' in self.debugged):
            self.logger.debug('Feedback: {}'.format(feedback_msg.process_feed))

        goal_handle.succeed()
        result = Led.Result()
        return result

def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    DEBUGGED = getenv('DEBUGGED').split(',')
    LEDMAIN_PIN = int(getenv('LEDMAIN_PIN'))


    init(args=args)

    ledmain_action_server = LEDMainActionServer('led', LOGLEVEL, DEBUGGED, LEDMAIN_PIN)

    spin(ledmain_action_server)

    shutdown()

if __name__ == '__main__':
    main()
