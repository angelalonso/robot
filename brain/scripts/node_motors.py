#!/usr/bin/env python3

import RPi.GPIO as GPIO
#from fake_rpi import fake_rpi as GPIO

class MotorsSubscriber(Node):
    def __init__(self):
        self.subscription = self.create_subscription(
                String,
                'motors_topic',
                self.subscriber_callback,
                10
                )
        self.subscription

    def subscriber_callback(self, msg):
        self.get_logger().info('Motors Topic: "%s"' % msg.data)
        # TODO: translate message into movement

def main(args=None):
    rclpy.init(args=args)
    motors_subscriber = MotorsSubscriber()

    rclpy.spin(input_subscriber)

    motors_subscriber.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()
