#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from std_msgs.msg import String
import RPi.GPIO as GPIO
#from fake_rpi import fake_rpi as GPIO

class MotorPublisher(Node):

    def __init__(self):
        super().__init__('motor_publisher')
        self.motor_status = 'stop'
        self.publisher_ = self.create_publisher(
                String, 
                'motors_topic', 
                10
                )
        timer_period = 0.5  # seconds
        self.timer = self.create_timer(timer_period, self.timer_callback)
        self.i = 0

    def timer_callback(self):
        # temporary
        if self.i % 2 == 0:
            self.motor_status = 'stop'
        elif self.i % 2 == 1:
            self.motor_status = 'run'
        msg = String()
        msg.data = '%s' % self.motor_status
        self.publisher_.publish(msg)
        self.get_logger().info('Publishing: "%s"' % msg.data)
        self.i += 1


def main(args=None):
    rclpy.init(args=args)
    motor_publisher = MotorPublisher()

    rclpy.spin(motor_publisher)

    motor_publisher.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()
