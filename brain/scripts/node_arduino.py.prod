#!/usr/bin/env python3

import os
import serial
import sys
import time

import rclpy
from rclpy.node import Node
from processing import process_input

class SerialLink(Node):
    def __init__(self):
        super().__init__('arduino_serial_sync')
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
                out += self.conn.read(1).decode()
            if out != '':
                self.get_logger().info(process_input(out))
                self.get_logger().info(process_input(self.latest_infos, out))
            time.sleep(1)

def main(args=None):
    rclpy.init(args=args)

    arduino_serial = SerialLink()

    arduino_serial.sync_and_read()

    rclpy.shutdown()

if __name__ == '__main__':
    main()
