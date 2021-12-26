#!/usr/bin/env python3

from interfaces.srv import SetStatus

from rclpy import init, logging, shutdown, ok, spin_once
from rclpy.node import Node

from dotenv import load_dotenv
from os import getenv
from os import path
from std_msgs.msg import String
import serial
import sys
import time

class SerialLink(Node):
    def __init__(self, loglevel, mode, mockfile, usb_dev):
        super().__init__('arduino_serial_sync')

        self.setstatus_cli = self.create_client(SetStatus, 'setstatus')
        while not self.setstatus_cli.wait_for_service(timeout_sec=1.0):
            self.get_logger().info('service not available, waiting again...')
        self.setstatus_req = SetStatus.Request()

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

    def send_setstatus_req(self, key, value):
        self.setstatus_req.key = key
        self.setstatus_req.value = value 
        self.future = self.setstatus_cli.call_async(self.setstatus_req)

    def sanitize(self, text):
        result = {}
        if text[0:8] == 'SENSOR: ':
            sensors = text.replace('\n', '').replace('SENSOR: ', '').split('|')
            for sensor_raw in sensors:
                if sensor_raw != '':
                    sensor = sensor_raw.split('=')
                    result[sensor[0]] = sensor[1]
        return result

    def send_from_sanitized(self, text):
        keyvals = self.sanitize(text)
        for key in keyvals:
            self.send_setstatus_req(key, keyvals[key])
            while ok():
                spin_once(self)
                if self.future.done():
                    try:
                        response = self.future.result()
                    except Exception as e:
                        self.get_logger().info(
                            'Service call failed %r' % (e,))
                    else:
                        self.get_logger().info(
                            'Result = %s' %
                            (response.current_status))
                    break

    def sync_and_read(self, read_delay):
        # On Mock Mode we will use our local file instead
        if self.mode == "mock":
            while True:
                out = self.mockfile.readline()
                self.get_logger().info(str(out))
                msg = String()
                msg.data = str(out)

                self.publisher_.publish(msg)
                self.send_from_sanitized(msg.data)

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
                    self.send_from_sanitized(msg.data)
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
