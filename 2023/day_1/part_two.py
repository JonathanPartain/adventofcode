#!/usr/bin/env python3

#input = "small.txt"
input = "input.txt"
int_val = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9"
}
sum = 0
with open(input, "r") as f:
    for line in f.readlines():
        index = 0
        line_order = []
        for c in line:
            if c.isdigit(): # if number, add to list
                line_order.append(c)
            else: # character
                for n in int_val.keys(): # check all keys
                    if n.startswith(c) and len(line) >= index + len(n): # make sure substring fits, and same start char for some speed
                        # check substring
                        if n == line[index:index+len(n)]:  # if key is same as substring, add the value. In this case integer as string
                            line_order.append(int_val[n])
            index += 1
        string_total = line_order[0] + line_order[-1]
        sum += int(string_total)

print(sum)



