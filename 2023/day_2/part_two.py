#!/usr/bin/env python3

# per game
# 12 red
# 13 green
# 14 blue

key = {
    "red": 12,
    "green": 13,
    "blue": 14
}
input_file = "input.txt"
sum = 0
with open(input_file, "r") as f:
    for line in f.readlines():
        # parse
        game, subsets = line.split(":")
        rolls = subsets.split(";")
        works = True
        g, id = game.split(" ")
        min_r = 0
        min_g = 0
        min_b = 0
        for roll in rolls:
            items = roll.split(",")
            for i in items:
                i = i.strip()
                v, k = i.split(" ")
                v = int(v)
                if k == "red":
                    if v > min_r:
                        min_r = v
                if k == "blue":
                    if v > min_b:
                        min_b = v
                if k == "green":
                    if v > min_g:
                        min_g = v
        print(f"red: {min_r}, green: {min_g}, blue: {min_b}")
        print(subsets)
        # min of all
        power = min_r * min_g * min_b
        sum += power
print(sum)


