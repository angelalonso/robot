#!/usr/bin/env python3

from rclpy import init
from rclpy import shutdown
from rclpy import logging
from rclpy.node import Node

from std_msgs.msg import String
from dotenv import load_dotenv
from os import getenv
from os import path
import serial
import sys
import time

class SerialLink(Node):
    def __init__(self, loglevel, mode, mockfile, usb_dev):
        super().__init__('arduino_serial_sync')
        self.mode = mode
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        self.publisher_ = self.create_publisher(String, 'main_topic', 10)
        # On Mock Mode we will use a local file as input
        if self.mode == "mock":
            try:
                self.mockfile = open(mockfile, "r") 
            except (FileNotFoundError, TypeError):
                self.mockfile = open("mock.log", "r") 
        else:
            # On Record Mode we will save whatever comes from the Arduino into a local file
            if self.mode == "record":
                try:
                    self.mockfile = open(mockfile, "w+") 
                except (FileNotFoundError, TypeError):
                    self.mockfile = open("mock.log", "w+") 
            portfound = False
            for portfile in [usb_dev, '/dev/ttyUSB0', '/dev/ttyACM0']:
                if (path.exists(portfile) and not portfound):
                    self.conn = serial.Serial(
                        port=portfile,
                        baudrate=9600,
                        parity=serial.PARITY_ODD,
                        stopbits=serial.STOPBITS_TWO,
                        bytesize=serial.SEVENBITS
                    )
                    self.conn.isOpen()
                    portfound = True
            if not portfound:
                self.get_logger().warn("ATTENTION: Could not connect to any USB-ACM ports")
                sys.exit(2)

            #TODO: need to close the connection somehow
                    #self.conn.close()

    def sync_and_read(self, read_delay):
        # On Mock Mode we will use our local file instead
        if self.mode == "mock":
            while True:
                out = self.mockfile.readline()
                self.get_logger().info(str(out))
                msg = String()
                msg.data = str(out)
                self.publisher_.publish(msg)
                time.sleep(read_delay)
        else:
            while True:
                out = ''
                self.conn.write(str.encode('\r\n'))
                while self.conn.inWaiting() > 0:
                    try:
                        out += self.conn.read(1).decode()
                    except UnicodeDecodeError:
                        pass
                if out != '':
                    self.get_logger().info(str(out))
                    msg = String()
                    msg.data = str(out)
                    if self.mode == "record":
                        self.mockfile.write(str(msg.data))
                    self.publisher_.publish(msg)
                time.sleep(read_delay)

def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')
    ARDUINO_MODE = getenv('ARDUINO_MODE')
    ARDUINO_MOCK_FILE = getenv('ARDUINO_MOCK_FILE')
    ARDUINO_USB_DEV = getenv('ARDUINO_USB_DEV')
    ARDUINO_READ_DELAY = float(getenv('ARDUINO_READ_DELAY'))

    init(args=args)

    arduino_serial = SerialLink(LOGLEVEL, 
            ARDUINO_MODE, 
            ARDUINO_MOCK_FILE, 
            ARDUINO_USB_DEV)

    arduino_serial.sync_and_read(ARDUINO_READ_DELAY)

    shutdown()

if __name__ == '__main__':
    main()
