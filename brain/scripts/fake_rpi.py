class fake_rpi(object):
    def __init__(self):
        super().__init__('fake rpi')
        self.setwarning = False

    def add_event_detect(pin_nr, state, callback):
        pass

    def setwarnings(state):
        pass

    def setmode(state):
        pass

    def setup(pin, mode, *args, **kwargs):
        pass

    def output(pin, mode):
        pass

    def BCM():
        return ""

    def BOARD():
        return ""

    def HIGH():
        return ""

    def IN():
        return ""

    def LOW():
        return ""

    def OUT():
        return ""

    def PWM(pin, mode):
        p = PWM_object()
        return p

    def PUD_DOWN():
        return ""

    def RISING():
        return ""

    def cleanup():
        pass

    def wait_for_edge(rising, timeout):
        from random import seed
        from random import randint
        from time import sleep
        import time
        seed(round(time.time() * 1000))
        sleepytime = randint(0, timeout * 2)
        if sleepytime > timeout:
            sleep(timeout / 1000)
            return None
        else:
            sleep(sleepytime / 1000)
            return ""

class PWM_object:
    #reusing for fake_rpi and fake_pigpio
    def __init__(self):
        pass

    def start(self, state):
        pass

    def ChangeDutyCycle(self, state):
        pass

    def set_mode(self, servo, mode):
        pass

    def set_PWM_frequency(self, pin, frequency):
        pass

    def set_PWM_dutycycle(self, pin, cycle):
        pass

    def set_servo_pulsewidth(self, pin, state):
        pass

class fake_pigpio(object):
    def __init__(self):
        super().__init__('fake pigpio')

    def pi():
        p = PWM_object()
        return p

    def OUTPUT():
        return ""

import sys
sys.modules["package.module"] = fake_rpi
