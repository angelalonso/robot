#!/usr/bin/env python3

from rclpy import init
from rclpy import shutdown
from rclpy import logging
from rclpy.node import Node
from processing import process_input
from processing import info_entries
from std_msgs.msg import String

from os import getenv
from os import path
import serial
import sys
import time

class SerialLink(Node):
    def __init__(self, loglevel):
        super().__init__('arduino_serial_sync')
        logging._root_logger.set_level(getattr(logging.LoggingSeverity, loglevel.upper()))
        self.publisher_ = self.create_publisher(String, 'main_topic', 10)
        self.latest_infos = info_entries()
        portfound = False
        for portfile in [ '/dev/ttyACM0', '/dev/ttyACM0']:
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
            print("ERROR: Could not connect to any USB-ACM ports")
            sys.exit(2)

        #TODO: need to close the connection somehow
                #self.conn.close()

    def sync_and_read(self):
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
                msg.data = process_input(self.latest_infos, out)
                self.publisher_.publish(msg)
            time.sleep(0.08)

def main(args=None):
    load_dotenv()
    LOGLEVEL = getenv('LOGLEVEL')

    init(args=args)

    arduino_serial = SerialLink(LOGLEVEL)

    arduino_serial.sync_and_read()

    shutdown()

if __name__ == '__main__':
    main()
