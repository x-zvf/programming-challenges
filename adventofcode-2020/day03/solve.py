#!/bin/env python
import functools
with open("input") as f:
    lines = [x.strip() for x in f.readlines()]


def count_slope(xyincs):
    xinc, yinc = xyincs
    x = 0
    y = 0
    count = 0
    while y < len(lines):
        if lines[y][x] == "#":
            count+=1
        x+=xinc
        x=x%len(lines[0])
        y+=yinc
    return count
counts = list(map(count_slope, [(1,1),(3,1),(5,1), (7,1),(1,2)]))
prod = functools.reduce(lambda a,b: a * b, counts)
print(counts, prod)

