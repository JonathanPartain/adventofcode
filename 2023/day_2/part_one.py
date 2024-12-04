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
        for roll in rolls:
            items = roll.split(",")
            for i in items:
                i = i.strip()
                print(i.split(" "))
                num, t = i.split(" ")
                if int(num) > key[t]:
                    works = False
        if works:
            sum += int(id)
print(sum)


