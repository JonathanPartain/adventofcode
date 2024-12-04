#!/usr/bin/env python3


up = "("
down = ")"

floor = 0
with open("input.txt", "r") as f:
    for line in f.readlines():
        for c in line:
            if c == up:
                floor += 1
            if c == down:
                floor -= 1
print(floor)

