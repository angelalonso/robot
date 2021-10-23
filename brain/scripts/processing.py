from collections import deque

class info_entries: 
    def __init__(self): 
        self.entries_list = deque()

    def append(self, entry):
        if len(self.entries_list) < 10:
            self.entries_list.appendleft(entry)
        else:
            self.entries_list.pop()
            self.entries_list.appendleft(entry)

    def printout(self):
        #return str(len(self.entries_list))
        return '|-|'.join(self.entries_list)


class info_entry: 
    def __init__(self, source, topic, value): 
        self.source = source 
        self.topic = topic 
        self.value = value 

    def __repr__(self):
        return 'source: ' + self.source + ', topic: ' + self.topic + ", value: " + self.value

def understand(latest_infos, data):
    info_list = data.split("|")
    info_correct = False
    list_ix = 0
    info_list_clean = [] 
    for info in info_list:
        try:
            source = info.split(': ')[0]
            try:
                topic = info.split(': ')[1].split('=')[0]
                try:
                    value = info.split(': ')[1].split('=')[1]
                    #TODO: do different stuff for different sources
                    #TODO: do different stuff for different topics
                    if (source == 'SENSOR') and (topic == 'distance') and (value.isnumeric()):
                        info_list_clean.append(info_entry(source, topic, value))
                        latest_infos.append(info_entry(source, topic, value))
                except IndexError:
                    pass
            except IndexError:
                pass
        except IndexError:
            pass
    return info_list_clean

def get_actions(latest_infos, data):
    info_list = understand(latest_infos, data)
    determination_file = open("determinations.list", "r")
    determinations = determination_file.read().split("\r")
    determination_file.close()
    action_list = []
    for entry in info_list:
        #TODO: after reading determinations, build a set of rules
        if int(entry.value) < 20:
            action_list.append("MOTOR: left=-100")
            action_list.append("MOTOR: right=-100")
        elif int(entry.value) < 30:
            action_list.append("MOTOR: left=-100")
            action_list.append("MOTOR: right=100")
        else:
            action_list.append("MOTOR: left=100")
            action_list.append("MOTOR: right=100")
    return '*'.join(action_list)
    #eturn '*'.join(determinations)

def get_test(latest_infos, data):
    latest_infos.append(data)
    return latest_infos.printout()

def process_input(latest_infos, data):
    return get_actions(latest_infos, data)
