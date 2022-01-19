#!/usr/bin/env python3

from rclpy import init, logging, shutdown, spin, spin_once, ok
from rclpy.action import ActionServer
from rclpy.node import Node
try:
    import pigpio
except ModuleNotFoundError:
    from lib_fake_rpi import fake_pigpio as pigpio

from interfaces.srv import GetStatusKey
from brain.action import Servo, Getstatus

from dotenv import load_dotenv
from os import getenv
import time

class ServoLaserActionServer(Node):

    def __init__(self, loglevel, enable):
        super().__init__('servolaser_action_server')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))

        self.getstatuskey_cli = self.create_client(GetStatusKey, 'getstatuskey')
        while not self.getstatuskey_cli.wait_for_service(timeout_sec=1.0):
            self.get_logger().info('service not available, waiting again...')
        self.getstatuskey_req = GetStatusKey.Request()
        self.client_futures = []

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
 
        self.get_logger().info('LASER SERVO: {}'.format(goal_handle.request.rotation))
        # my own standard: 0 means we do a scan
        if goal_handle.request.rotation == 0:
            self.scan_front()
        else:
            self.do_rotate(goal_handle.request.rotation)
        if self.state != goal_handle.request.rotation:
            self.state = goal_handle.request.rotation
            self.get_logger().info('Feedback: {}'.format(feedback_msg.process_feed))

        goal_handle.succeed()
        result = Servo.Result()
        return result

    def send_getstatuslaser_req(self):
        key = 'laser'
        self.getstatuskey_req.key = key
        self.future = self.getstatuskey_cli.call_async(self.getstatuskey_req)

    def send_getstatuslaser(self):
        key = 'laser'
        self.send_getstatuslaser_req()
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

    def do_rotate(self, rotation):
        self.state = rotation
        self.pwm.set_servo_pulsewidth(self.pin, self.state)

    def scan_front(self):
        self.state = 1400
        self.pwm.set_servo_pulsewidth(self.pin, self.state)
        # TODO: 
        # once this is run, we cannot trigger it again
        # https://gist.github.com/driftregion/14f6da05a71a57ef0804b68e17b06de5
        aux = self.send_getstatuslaser()
        self.get_logger().info('LASER VALUE: {}'.format(aux))
        #self.get_logger().info('LASER VALUE: {}'.format(self.send_getstatuslaser()))
        #time.sleep(0.5)
        #self.state = 1600
        #self.pwm.set_servo_pulsewidth(self.pin, self.state)
        #self.get_logger().info('LASER VALUE: {}'.format(self.send_getstatuslaser()))
        #time.sleep(0.5)
        #self.state = 1500
        #self.get_logger().info('LASER VALUE: {}'.format(self.send_getstatuslaser()))
        #self.pwm.set_servo_pulsewidth(self.pin, self.state)
        #print("cleaned")
        #self.stop()

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
        #print("cleaned")
        #self.stop()

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
