#!/usr/bin/env python3

from rclpy import init, logging, shutdown, spin
from rclpy.action import ActionServer
from rclpy.node import Node
try:
    import pigpio
except ModuleNotFoundError:
    from fake_rpi import fake_pigpio as pigpio

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
            self.pwm = pigpio.pi() 
            self.pwm.set_mode(self.pin, pigpio.OUTPUT)

            self.pwm.set_PWM_frequency(self.pin, 50) # GPIO 18 for PWM with 50Hz
            self.state = 1500
            self.pwm.set_servo_pulsewidth(self.pin, self.state)

            self._action_server = ActionServer(
                self,
                Servo,
                'ServoLaser',
                self.execute_callback)
        else:
            self.get_logger().info('SERVO FOR LASER DISABLED')

    def execute_callback(self, goal_handle):
        feedback_msg = Servo.Feedback()
        goal_handle.publish_feedback(feedback_msg)
 
        self.get_logger().info('TEST: {}'.format(goal_handle.request.rotation))
        if goal_handle.request.rotation < 0:
            feedback_msg.process_feed = "moving Servo for Laser SCAN"
            self.scan_loop()
        else:
            feedback_msg.process_feed = "moving Servo for Laser " + str(goal_handle.request.rotation)
            self.state = goal_handle.request.rotation
            self.pwm.set_servo_pulsewidth(self.pin, self.state)

        if self.state != goal_handle.request.rotation:
            self.state = goal_handle.request.rotation
            self.get_logger().info('Feedback: {}'.format(feedback_msg.process_feed))

        goal_handle.succeed()
        result = Servo.Result()
        return result

    def scan_loop(self):
        try:
            for dc in range(500, 2501, 100):
                self.state = dc
                self.pwm.set_servo_pulsewidth(self.pin, self.state)
                time.sleep(0.5)
            for dc in range(2500, 499, -100):
                self.state = dc
                self.pwm.set_servo_pulsewidth(self.pin, self.state)
                time.sleep(0.5)
        except KeyboardInterrupt:
            pass
        print("cleaned")
        self.stop()

    def stop(self):
        self.pwm.set_PWM_dutycycle(self.pin, 0)
        self.pwm.set_PWM_frequency(self.pin, 0)

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
