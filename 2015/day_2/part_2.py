#!/usr/bin/env python3

total = 0
with open("input.txt", "r") as f:
    for line in f.readlines():
        l, w, h = line.split("x")
        l = int(l)
        w = int(w)
        h = int(h)
        s1, s2, _ = sorted([l,w,h])
        bow = s1 + s1 + s2 + s2
        wrap = l * w * h
        this_total = bow + wrap
        total += this_total
print(total)

