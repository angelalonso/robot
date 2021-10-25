import csv
import sys

def read_rules(filename):
    csvfile = open(filename, newline='')
    dictreader = csv.DictReader(csvfile, delimiter=';')
    return dictreader

def find_match(dataset, new_input):
    # TODO: open this up for more than one input variable
    prev_entry = None
    prev_diff = None
    next_entry = None
    next_diff = None
    for entry in dataset:
    # If there's a direct match, get the result
        if entry['distance'] == new_input:
            return entry['speed']
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
    
if __name__ == '__main__':
    data = read_rules('rules.csv')
    print(find_match(data, sys.argv[1]))

