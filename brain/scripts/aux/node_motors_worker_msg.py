#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from std_msgs.msg import String
import RPi.GPIO as GPIO
#from fake_rpi import fake_rpi as GPIO

class Motors:
    def __init__(self):
        self.right_in1 = 24
        self.right_in2 = 23
        self.right_en = 25
        self.right_state = 1
        
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(right_in1,GPIO.OUT,None)
        GPIO.setup(right_in2,GPIO.OUT,None)
        GPIO.setup(right_en,GPIO.OUT,None)
        GPIO.output(right_in1,GPIO.LOW)
        GPIO.output(right_in2,GPIO.LOW)

        self.p = GPIO.PWM(right_en,1000)
        self.p.start(100)

    def run(self):
        print("run")
        if(right_state==1):
            GPIO.output(self.right_in1,GPIO.HIGH)
            GPIO.output(self.right_in2,GPIO.LOW)
            print("forward")
        else:
            GPIO.output(self.right_in1,GPIO.LOW)
            GPIO.output(self.right_in2,GPIO.HIGH)
            print("backward")

    def stop(self):
        print("stop")
        GPIO.output(self.right_in1,GPIO.LOW)
        GPIO.output(self.right_in2,GPIO.LOW)


class MotorsSubscriber(Node):

    def __init__(self):
        super().__init__('motor_subscriber')
        self.subscription = self.create_subscription(
                String,
                'motors_topic',
                self.subscriber_callback,
                10)
        self.subscription

    def subscriber_callback(self, msg):
        self.get_logger().info('Motors Topic: "%s"' % msg.data)
        # TODO: translate message into movement

def main(args=None):
    rclpy.init(args=args)
    motors_subscriber = MotorsSubscriber()

    rclpy.spin(motors_subscriber)

    motors_subscriber.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()
