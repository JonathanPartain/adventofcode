#!/usr/bin/env python3

input_file = "input.txt"
values = []
sum = 0
with open(input_file, "r") as f:
    for line in f.readlines():
        # get list of numbers
        for s in line:
            if s.isdigit():
                values.append(int(s))
        # build number
        new_number = str(values[0]) + str(values[-1])
        sum += int(new_number)
        values = []
print(sum)
