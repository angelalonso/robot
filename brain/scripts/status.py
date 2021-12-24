class Status(object):
    def __init__(self):
        super().__init__()
        self.current = {}

    def __getitem__(self, item):
         return self.current[item]

    def set_status(self, element, value):
        self.current[element] = value
