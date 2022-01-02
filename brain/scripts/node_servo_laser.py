#!/usr/bin/env python3

from rclpy import init, logging, shutdown, spin
from rclpy.action import ActionServer
from rclpy.node import Node
try:
    import RPi.GPIO as GPIO
except ModuleNotFoundError:
    from fake_rpi import fake_rpi as GPIO

from brain.action import Servo

from dotenv import load_dotenv
from os import getenv
import time

class ServoLaserActionServer(Node):

    def __init__(self, loglevel, enable):
        super().__init__('servolaser_action_server')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        if enable:
            self.pin = 18
            GPIO.setmode(GPIO.BCM)
            GPIO.setup(self.pin, GPIO.OUT)

            self.pwm = GPIO.PWM(self.pin, 500) # GPIO 18 for PWM with 50Hz
            self.correction = 0 # needed?
            self.state = 0
            self.pwm.start(0) # Initialization
            #self.pwm.ChangeDutyCycle(30)
            #self.pwm.stop()
            #GPIO.cleanup()

            self._action_server = ActionServer(
                self,
                Servo,
                'ServoLaser',
                self.execute_callback)
        else:
            self.get_logger().info('SERVO FOR LASER DISABLED')

    def execute_callback(self, goal_handle):
        self.pwm.start(self.state) # Initialization
        feedback_msg = Servo.Feedback()
        goal_handle.publish_feedback(feedback_msg)
 
        self.get_logger().info('TEST: {}'.format(goal_handle.request.rotation))
        if goal_handle.request.rotation < 0:
            feedback_msg.process_feed = "moving Servo for Laser SCAN"
            self.scan_loop()
        else:
            feedback_msg.process_feed = "moving Servo for Laser " + str(goal_handle.request.rotation)
            self.pwm.ChangeDutyCycle(goal_handle.request.rotation + self.correction)

        if self.state != goal_handle.request.rotation:
            self.state = goal_handle.request.rotation
            self.get_logger().info('Feedback: {}'.format(feedback_msg.process_feed))
        #self.pwm.stop()
        #GPIO.cleanup()

        goal_handle.succeed()
        result = Servo.Result()
        return result

    def scan_loop(self):
        try:
            print("first")
            for dc in range(40, 101, 5):
                self.pwm.ChangeDutyCycle(dc)
                time.sleep(0.5)
            print("second")
            for dc in range(100, 35, -5):
                self.pwm.ChangeDutyCycle(dc)
                time.sleep(0.5)
        except KeyboardInterrupt:
            pass
        print("cleaned")
        p.stop()
        GPIO.cleanup()

    def stop(self):
        self.pwm.stop()
        GPIO.cleanup()

def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    ENABLE = getenv('ENABLE_SERVO_LASER')

    init(args=args)

    servolaser_action_server = ServoLaserActionServer(LOGLEVEL, ENABLE)

    spin(servolaser_action_server)

    shutdown()

if __name__ == '__main__':
    main()
