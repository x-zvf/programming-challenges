#!/bin/env python3
import math

with open("input") as f:
    lines = f.readlines()

def calc_seat(s):
    row_s = s[0:7]
    col_s = s[7:10]

    min_r = 0
    max_r = 127

    for c in row_s:
        #print("c:",c,"min:",min_r,"max:", max_r)
        if c == "B":
            min_r += math.ceil((max_r - min_r) / 2)
        elif c == "F":
            max_r -= math.ceil((max_r - min_r) / 2)

    #print(min_r, max_r)
    assert min_r == max_r

    row = min_r

    min_c = 0
    max_c = 7
    for c in col_s:
        if c == "R":
            min_c += math.ceil((max_c - min_c) / 2)
        elif c == "L":
            max_c -= math.ceil((max_c - min_c) / 2)
    assert min_c == max_c
    col = min_c
    sid = (row * 8) + col
    return row, col, sid

seats = list(map(calc_seat, lines))

seat_ids = [x[2] for x in seats]

print(max(seats, key=lambda x: x[2]))

print(next(filter(lambda x: (x not in seat_ids) and (x-1 in seat_ids) and (x+1 in seat_ids), range(1024))))
