#!/usr/bin/env python3

from random import seed
from random import randint
import time
import hashlib

class Arduino():

    def __init__(self):
        self.infile = "mock_arduino_serial.input"
        try:
            open(self.infile, 'x')
        except FileExistsError:
            pass

        self.inobj = open(self.infile, 'rb')
        inobj_md5 = hashlib.md5()
        content = self.inobj.read()
        inobj_md5.update(content)
        self.inobj_hash = inobj_md5.hexdigest()
        self.outfile = "mock_arduino_serial.output"
        try:
            open(self.outfile, 'x')
        except FileExistsError:
            pass
        self.outobj = open(self.outfile, 'rb')
        outobj_md5 = hashlib.md5()
        content = self.outobj.read()
        outobj_md5.update(content)
        self.outobj_hash = outobj_md5.hexdigest()

    def write(self):
        seed(round(time.time() * 1000))
        mocked_distance = randint(0, 999)
        with open(self.outfile, "w") as outfile:
            outfile.write("SENSOR: distance=" + str(mocked_distance) + "|")

    def sync(self):
        while True:
            aux_inobj = open(self.infile, 'rb')
            aux_inobj_md5 = hashlib.md5()
            content = aux_inobj.read()
            aux_inobj_md5.update(content)
            new_inobj_hash = aux_inobj_md5.hexdigest()
            
            if self.inobj_hash != new_inobj_hash:
                print("SYNCFILE CHANGED!")
                self.inobj_hash = new_inobj_hash
                self.write()
            time.sleep(1)


def main(args=None):
    arduino = Arduino()
    arduino.sync()

if __name__ == '__main__':
    main()
