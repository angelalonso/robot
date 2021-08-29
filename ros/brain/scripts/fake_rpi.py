class fake_rpi(object):
    def __init__(self):
        super().__init__('fake rpi')
        self.setwarning = False

    def setwarning(state):
        pass

    def setmode(state):
        pass

    def setup(pin, mode):
        pass

    def output(pin, mode):
        pass

    def BCM():
        return ""

    def OUT():
        return ""

    def HIGH():
        return ""

    def LOW():
        return ""

import sys
sys.modules["package.module"] = fake_rpi
