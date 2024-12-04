#!/usr/bin/env python3
import re

input_file = "input.txt"
#input_file = "small.txt"

numbers = r"[0-9]+"
not_num = r"[*]"
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

s = 0
def numbers_next_to_gear(line_num, gear_location):
    numbers = []
    pre_line = line_num-1
    post_line = line_num+1

    # get range, span-1 to span+1 
    # line above
    try:
        line_vals = num_locations[pre_line]
        for k, val in line_vals.items():
            index_low, index_high = k[0],k[1]
            for l in range(index_low-1, index_high+1):
                if gear_location == l:
                    numbers.append(val)
    except:
        print("except above")
        pass
    # same line
    try:
        line_vals = num_locations[line_num]
        for k, val in line_vals.items():
            index_low, index_high = k[0],k[1]
            for l in range(index_low-1, index_high+1):
                if gear_location == l:
                    numbers.append(val)
    except:
        print("except same")
        pass
    
    try:
        line_vals = num_locations[post_line]
        for k, val in line_vals.items():
            index_low, index_high = k[0],k[1]
            for l in range(index_low-1, index_high+1):
                if gear_location == l:
                    numbers.append(val)
    except:
        print("except below")
        pass
    return numbers

sum = 0
for i, num in char_locations.items():

    for k, v in num.items():
        index = k[0]
        a = numbers_next_to_gear(i, index)
        print(a)
        if len(a) == 2:
            p1 = int(a[0])
            p2 = int(a[1])
            ratio = p1 * p2
            sum += ratio
print(sum)
