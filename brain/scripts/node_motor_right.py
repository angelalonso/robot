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

class MotorRightActionServer(Node):

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
        if ('all' in self.debugged) or ('motor_r' in self.debugged) or ('motorr' in self.debugged) or ('motor_right' in self.debugged):
            self.logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        #logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))

        self.right_in1 = int(pin_in1)
        self.right_in2 = int(pin_in2)
        self.right_en = int(pin_ena)
        self.state = "Stop"
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(self.right_in1,GPIO.OUT)
        GPIO.setup(self.right_in2,GPIO.OUT)
        GPIO.setup(self.right_en,GPIO.OUT)
        GPIO.output(self.right_in1,GPIO.LOW)
        GPIO.output(self.right_in2,GPIO.LOW)

        self.p = GPIO.PWM(self.right_en,1000)
        self.p.start(100 * power_factor)

        self._action_server = ActionServer(
            self,
            Motor,
            'MotorRight',
            self.execute_callback)

    def execute_callback(self, goal_handle):
        feedback_msg = Motor.Feedback()
        goal_handle.publish_feedback(feedback_msg)

        if goal_handle.request.move == "Forward":
            feedback_msg.process_feed = "moving Right Motor " + str(goal_handle.request.move)
            GPIO.output(self.right_in1,GPIO.LOW)
            GPIO.output(self.right_in2,GPIO.HIGH)
        elif goal_handle.request.move == "Backward":
            feedback_msg.process_feed = "moving Right Motor " + str(goal_handle.request.move)
            GPIO.output(self.right_in1,GPIO.HIGH)
            GPIO.output(self.right_in2,GPIO.LOW)
        else:
            feedback_msg.process_feed = "NOT moving Right Motor because movement: " + str(goal_handle.request.move)
            GPIO.output(self.right_in1,GPIO.LOW)
            GPIO.output(self.right_in2,GPIO.LOW)

        if self.state != goal_handle.request.move:
            self.state = goal_handle.request.move
            if ('all' in self.debugged) or ('motor_r' in self.debugged) or ('motorr' in self.debugged) or ('motor_right' in self.debugged):
                self.logger.debug('Feedback: {}'.format(feedback_msg.process_feed))

        goal_handle.succeed()
        result = Motor.Result()
        return result

def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    raw_debugged = getenv('DEBUGGED')
    try:
        DEBUGGED = str(raw_debugged).split(',')
    except:
        DEBUGGED = []
    MOTOR_R_PIN_IN1 = getenv('MOTOR_R_PIN_IN1')
    MOTOR_R_PIN_IN2 = getenv('MOTOR_R_PIN_IN2')
    MOTOR_R_PIN_ENA = getenv('MOTOR_R_PIN_ENA')
    raw_motor_r_factor = getenv('MOTOR_R_FACTOR')
    try:
        MOTOR_R_FACTOR = float(str(raw_motor_r_factor))
    except:
        MOTOR_R_FACTOR = 1.0

    init(args=args)

    motorright_action_server = MotorRightActionServer('motor_right', 
            LOGLEVEL, 
            DEBUGGED,
            MOTOR_R_PIN_IN1,
            MOTOR_R_PIN_IN2,
            MOTOR_R_PIN_ENA,
            MOTOR_R_FACTOR)

    spin(motorright_action_server)

    shutdown()

if __name__ == '__main__':
    main()
