#!/usr/bin/env python3

import RPi.GPIO as GPIO
#from fake_rpi import fake_rpi as GPIO

class InputSubscriber(Node):
    def __init__(self):
        self.subscription = self.create_subscription(
                String,
                'input_topic',
                self.subscriber_callback,
                10
                )
        self.subscription

    def subscriber_callback(self, msg):
        self.get_logger().info('Input Topic: "%s"' % msg.data)

def main(args=None):
    rclpy.init(args=args)
    input_subscriber = InputSubscriber()

    rclpy.spin(input_subscriber)

    input_subscriber.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()
