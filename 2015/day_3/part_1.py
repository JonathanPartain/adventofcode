#!/usr/bin/env python3

visited = []
start_x = 0
start_y = 0
n = "^"
s = "v"
e = ">"
w = "<"

visited.append((start_x, start_y)) # start
with open("input.txt", "r") as f:
    for line in f.readlines():
        for c in line:
            #movement
            # goto 0,0 
            if c == n:
                start_y += 1
            if c == e:
                start_x += 1
            if c == s:
                start_y -= 1
            if c == w:
                start_x -= 1
            visited.append((start_x, start_y))
        print(visited)
        print(len(set(visited)))
        visited = []
        start_x = 0
        start_y = 0

