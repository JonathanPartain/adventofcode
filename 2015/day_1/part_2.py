#!/usr/bin/env python3


up = "("
down = ")"
# find floor -1
def find_basement():
    floor = 0
    with open("input.txt", "r") as f:
        for line in f.readlines():
            for pos,c in enumerate(line, start=1):
                if c == up:
                    floor += 1
                if c == down:
                    floor -= 1
                if floor == -1:
                    return pos
print(find_basement())

