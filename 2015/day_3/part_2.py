#!/usr/bin/env python3

visited = []
start_x = 0
start_y = 0

robo_x = 0
robo_y = 0

n = "^"
s = "v"
e = ">"
w = "<"

real_santa = True
with open("input.txt", "r") as f:
    for line in f.readlines():
        visited.append((start_x, start_y)) # start
        for c in line:
            if real_santa:
            #movement
                if c == n:
                    start_y += 1
                if c == e:
                    start_x += 1
                if c == s:
                    start_y -= 1
                if c == w:
                    start_x -= 1
                visited.append((start_x, start_y))
            else:
                if c == n:
                    robo_y += 1
                if c == e:
                    robo_x += 1
                if c == s:
                    robo_y -= 1
                if c == w:
                    robo_x -= 1
                visited.append((robo_x, robo_y))
            real_santa = not real_santa
        print(len(set(visited)))
        start_x = 0
        start_y = 0

        robo_x = 0
        robo_y = 0
        visited = []

