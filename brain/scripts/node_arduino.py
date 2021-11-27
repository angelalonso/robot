#!/usr/bin/env python3

import os
import serial
import sys
import time

import rclpy
from rclpy.node import Node
from processing import process_input
from processing import info_entries
from std_msgs.msg import String

class SerialLink(Node):
    def __init__(self):
        super().__init__('arduino_serial_sync')
        self.publisher_ = self.create_publisher(String, 'main_topic', 10)
        self.latest_infos = info_entries()
        portfound = False
        for portfile in [ '/dev/ttyACM0', '/dev/ttyACM0']:
            if (os.path.exists(portfile) and not portfound):
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
                msg = String()
                msg.data = process_input(self.latest_infos, out)
                self.publisher_.publish(msg)
            time.sleep(0.1)

def main(args=None):
    rclpy.init(args=args)

    arduino_serial = SerialLink()

    arduino_serial.sync_and_read()

    rclpy.shutdown()

if __name__ == '__main__':
    main()