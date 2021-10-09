def cleanup(msg):
    disallowed_characters = "."
    for character in disallowed_characters:
        msg = msg.replace(character, "").strip()

    result = msg
    return result


def process_input(msg):
    info_list = msg.split("|")
    info_correct = False
    list_ix = 0
    info = ''
    while not info_correct:
        info = cleanup(info_list[list_ix])
        list_ix += 1
        info_correct = True
        # TODO:
        # check which kind of info we get (e.g.: SENSOR)
        # then what specific info it is (e.g.: distance)
        # get the actual value
        # If all is OK, use just that one and discard the rest
    return ("processed " + info)
    #for info in info_list:
    #    result = "processed " + cleanup(msg)
    #return result
