
class info_entry: 
    def __init__(self, source, topic, value): 
        self.source = source 
        self.topic = topic 
        self.value = value 
    def __repr__(self):
        return 'source: ' + self.source + ', topic: ' + self.topic + ", value: " + self.value

def understand(data):
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
                except IndexError:
                    pass
            except IndexError:
                pass
        except IndexError:
            pass
    return info_list_clean

def get_actions(data):
    info_list = understand(data)
    action_list = []
    for entry in info_list:
        #TODO: play with values, past values etc...to do more complex actions
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

def process_input(data):
    return get_actions(data)
