#!/usr/bin/env python3

total = 0
with open("input.txt", "r") as f:
    for line in f.readlines():
        l, w, h = line.split("x")
        l = int(l)
        w = int(w)
        h = int(h)
        area_1 = 2 * l * w
        area_2 = 2 * w * h
        area_3 = 2 * h * l
        side_1 = l * w
        side_2 = l * h
        side_3 = w * h
        smallest = min([side_1, side_2, side_3])
        this_total = area_1 + area_2 + area_3 + smallest

        total += this_total

print(total)
