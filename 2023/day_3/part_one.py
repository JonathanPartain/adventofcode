#!/usr/bin/env python3
import re

input_file = "input.txt"
#input_file = "small.txt"

numbers = r"[0-9]+"
not_num = r"[^0-9\.\n]+"
index_line = 0
num_locations = {}
char_locations = {}
with open(input_file, "r") as f:
    for line in f.readlines():
        num_locations[index_line] = {}
        char_locations[index_line] = {}
        n = re.finditer(numbers, line)
        x = re.finditer(not_num, line)
        if n:
            for foo in n:
                span = foo.span()
                value = foo.group()
                num_locations[index_line][span] = value
        if x:
            for bar in x:
                span = bar.span()
                c = bar.group()
                char_locations[index_line][span] = c
        index_line += 1

def check_next_to(line_num, span):
    pre_line = line_num-1
    post_line = line_num+1
    span_outer = span[0]
    span_inner = span[1]
    span_min = span_outer - 1
    span_max = span_inner + 1

    # get range, span-1 to span+1 
    # line above
    try:
        line_vals = char_locations[pre_line]
        for k, _ in line_vals.items():
            index, _ = k[0],k[1]
            if span_min <= index < span_max:
                return True
    except:
        print("except above")
        pass
    # same line
    try:
        line_vals = char_locations[line_num]
        for k, _ in line_vals.items():
            index, _ = k[0],k[1]
            if span_min <= index < span_max:
                return True
    except:
        print("except same")
        pass

    # line below
    try:
        line_vals = char_locations[post_line]
        for k, _ in line_vals.items():
            index, _ = k[0],k[1]
            if span_min <= index < span_max:
                return True
    except:
        print("except below")
        pass
    return False

s = 0
for i, num in num_locations.items():
    for k, v in num.items():
        if check_next_to(i, k):
            print(f"value: {v}")
            s += int(v)
print(s)
