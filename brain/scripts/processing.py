
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
                    #TODO: check source is correct, do different stuff for different sources
                    #TODO: check topic is correct, do different stuff for different topics
                    if (source == 'SENSOR') and (topic == 'distance') and (value != ''):
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
        action_list.append(str(entry))
    return '*'.join(action_list)

def process_input(data):
    return get_actions(data)
