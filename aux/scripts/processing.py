import csv
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

def read_rules(filename):
    csvfile = open(filename, newline='')
    dictreader = csv.DictReader(csvfile, delimiter=';')
    return dictreader

def find_action(dataset, new_input):
    # TODO: open this up for more than one input variable
    prev_entry = None
    prev_diff = None
    next_entry = None
    next_diff = None
    for entry in dataset:
    # If there's a direct match, get the result
        if entry['distance'] == new_input:
            return entry[list(entry)[-1]]
    # If there isn't a direct match, find the prev and next values, 
    #   if they have the same result, that's our return
    #   if not, choose the prev over the next
        else:
            diff = int(entry['distance']) - int(new_input)
            if diff < 0:
                if prev_diff == None:
                    prev_diff = diff
                    prev_entry = entry
                elif diff > prev_diff:
                    prev_diff = diff
                    prev_entry = entry

            elif diff > 0:
                if next_diff == None:
                    next_diff = diff
                    next_entry = entry
                elif diff < next_diff:
                    next_diff = diff
                    next_entry = entry
    if prev_entry == None:
        if next_entry == None:
            return None
        else:
            return next_entry[list(next_entry)[-1]]
    else:
        return prev_entry[list(prev_entry)[-1]]

def get_actions(latest_infos, data):
    info_list = understand(latest_infos, data)
    rules_data = read_rules('rules.csv')
    action_list = []
    for entry in info_list:
        action_list.append("ACTIONS: " + find_action(rules_data, entry.value))
    return '|'.join(action_list)

def get_test(latest_infos, data):
    latest_infos.append(data)
    return latest_infos.printout()

def process_input(latest_infos, data):
    return get_actions(latest_infos, data)
