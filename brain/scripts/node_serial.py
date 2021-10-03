#!/usr/bin/env python3

import os
import serial
import sys
import time

class SerialLink():
    def __init__(self):
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

    def process(self, msg):
        #TODO: 
        # filter out noise
        # get one value on each run, or nothing (in the event of several, calculate median maybe?)
        print(msg)

    def read(self):
        while True:
            out = ''
            self.conn.write(str.encode('\r\n'))
            while self.conn.inWaiting() > 0:
                out += self.conn.read(1).decode()
            if out != '':
                self.process(out)
            time.sleep(1)

def main(args=None):
    serial = SerialLink()
    serial.read()

if __name__ == '__main__':
    main()
