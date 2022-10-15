#!/usr/bin/env python3

from rclpy import init, logging, shutdown, spin
from rclpy.action import ActionServer
from rclpy.node import Node
try:
    import RPi.GPIO as GPIO
except ModuleNotFoundError:
    from lib_fake_rpi import fake_rpi as GPIO

from brain.action import Motor

from dotenv import load_dotenv
from os import getenv

class MotorLeftActionServer(Node):

    def __init__(self, 
            name,
            loglevel, 
            debugged,
            pin_in1,
            pin_in2,
            pin_ena,
            power_factor):
        super().__init__(name)

        self.debugged = debugged 
        self.logger = logging.get_logger(name)
        if ('all' in self.debugged) or ('motor_l' in self.debugged) or ('motorl' in self.debugged) or ('motor_left' in self.debugged):
            self.logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        #logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))

        self.left_in1 = pin_in1
        self.left_in2 = pin_in2
        self.left_en = pin_ena
        self.state = "Stop"
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(self.left_in1,GPIO.OUT)
        GPIO.setup(self.left_in2,GPIO.OUT)
        GPIO.setup(self.left_en,GPIO.OUT)
        GPIO.output(self.left_in1,GPIO.LOW)
        GPIO.output(self.left_in2,GPIO.LOW)

        self.p = GPIO.PWM(self.left_en,1000)
        self.p.start(100 * power_factor)

        self._action_server = ActionServer(
            self,
            Motor,
            'MotorLeft',
            self.execute_callback)

    def execute_callback(self, goal_handle):
        feedback_msg = Motor.Feedback()
        goal_handle.publish_feedback(feedback_msg)

        if goal_handle.request.move == "Forward":
            feedback_msg.process_feed = "moving Left Motor " + str(goal_handle.request.move)
            GPIO.output(self.left_in1,GPIO.LOW)
            GPIO.output(self.left_in2,GPIO.HIGH)
        elif goal_handle.request.move == "Backward":
            feedback_msg.process_feed = "moving Left Motor " + str(goal_handle.request.move)
            GPIO.output(self.left_in1,GPIO.HIGH)
            GPIO.output(self.left_in2,GPIO.LOW)
        else:
            feedback_msg.process_feed = "NOT moving Left Motor because movement: " + str(goal_handle.request.move)
            GPIO.output(self.left_in1,GPIO.LOW)
            GPIO.output(self.left_in2,GPIO.LOW)

        if self.state != goal_handle.request.move:
            self.state = goal_handle.request.move
            if ('all' in self.debugged) or ('motor_l' in self.debugged) or ('motorl' in self.debugged) or ('motor_left' in self.debugged):
                self.logger.debug('Feedback: {}'.format(feedback_msg.process_feed))

        goal_handle.succeed()
        result = Motor.Result()
        return result

def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    DEBUGGED = getenv('DEBUGGED').split(',')
    MOTOR_L_PIN_IN1 = getenv('MOTOR_L_PIN_IN1')
    MOTOR_L_PIN_IN2 = getenv('MOTOR_L_PIN_IN2')
    MOTOR_L_PIN_ENA = getenv('MOTOR_L_PIN_ENA')
    MOTOR_L_FACTOR = float(getenv('MOTOR_L_FACTOR'))

    init(args=args)

    motorleft_action_server = MotorLeftActionServer('motor_left', 
            LOGLEVEL, 
            DEBUGGED,
            MOTOR_L_PIN_IN1,
            MOTOR_L_PIN_IN2,
            MOTOR_L_PIN_ENA,
            MOTOR_L_FACTOR)

    spin(motorleft_action_server)

    shutdown()

if __name__ == '__main__':
    main()
