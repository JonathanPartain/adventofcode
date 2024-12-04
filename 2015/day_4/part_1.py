#!/usr/bin/env python3
import hashlib

puzzle_input = "iwrupvqb"

h = hashlib.new("md5")
start_string = "00000"

def computeHash(string):
    m = hashlib.md5()
    m.update(string.encode('utf-8'))
    return m.hexdigest()

for i in range(1000000000):
    s = str(i)
    test = puzzle_input + s
    hash = computeHash(test)
    if hash.startswith(start_string):
        print(hash)
        print(i)
        exit(0)

