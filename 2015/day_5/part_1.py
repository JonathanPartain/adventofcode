#!/usr/bin/env python3

vowels = ["a", "e", "i", "o", "u"]
# same char 2x in a row
disallowed = ["ab", "cd", "pq", "xy"]

filename = "input.txt"

has_double = False
vowel_count = 0
allowed = True
with open(filename, "r") as f:
    nice = 0
    for line in f.readlines():
        allowed = True
        # contains disallowed string
        for x in disallowed:
            if x in line:
                allowed = False
        vowel_count = 0
        for c in line:
            if c in vowels:
                vowel_count += 1
        # double letter
        has_double = False
        for i, c in enumerate(line):
            if i + 1 < len(line):
                if c == line[i+1]:
                    has_double = True

        # check niceness
        if allowed and vowel_count >= 3 and has_double:
            print(line + " is nice")
            nice += 1
        else:
            print(line + " is naughty")
    print(nice)


                


