#!/usr/bin/env python3

filename = "input.txt"
filename = "small2.txt"

# contains a pair of any 2 letters at least 2x
def pair_twice(input_str):
    for i, _ in enumerate(input_str):
        if i+2<len(input_str):
            p = input_str[i:i+2]
            check_string = input_str[:i] + input_str[i+2:]
            if p in check_string:
                return True
    return False

# character repeats with exactly one letter in between
def find_repeat_char(input_str):
    for i, c in enumerate(input_str):
        if i+2<len(input_str):
            if c == input_str[i+2]:
                return True
    return False

with open(filename, "r") as f:
    nice_strings = 0
    for line in f.readlines():
        pair = pair_twice(line)
        #print(line)
        #print(str(pair) + " - pair")
        repeat = find_repeat_char(line)
        #print(str(repeat) + " - repeat")
        if pair and repeat:
            nice_strings += 1
    print(nice_strings)

