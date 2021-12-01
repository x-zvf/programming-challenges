#!/bin/env python3

with open("input") as f:
    lines = f.readlines()

heading = 1 # north = 0, east = 1, south = 2, west = 3
east=0
north=0

for line in lines:
    command = line[0]
    value = int(line[1:])
    print(f"[heading={heading}, north={north}, east={east}]({command}{value})", end="")
    if command == "N":
        north += value
    elif command == "S":
        north -= value
    elif command == "E":
        east += value
    elif command == "W":
        east -= value
    elif command == "R":
        mv = (value % 360) // 90
        heading += mv
        heading %= 4
    elif command == "L":
        mv = (value % 360) // 90
        heading -= mv
        heading %= 4
    elif command == "F":
        if heading == 0:
            north += value
        elif heading == 1:
            east += value
        elif heading == 2:
            north -= value
        elif heading == 3:
            east -= value
    print(f"---> [heading={heading}, north={north}, east={east}]")

print(north, east)
print(abs(north) + abs(east))

north = 0
east = 0
wp_north = 1
wp_east = 10


for line in lines:
    command = line[0]
    value = int(line[1:])
    print(f"[north={north}, east={east}, wp_north={wp_north}, wp_east={wp_east}]({command}{value})", end="")
    if command == "N":
        wp_north += value
    elif command == "S":
        wp_north -= value
    elif command == "E":
        wp_east += value
    elif command == "W":
        wp_east -= value
    elif command == "R":
        mv = (value % 360) // 90
        for _ in range(mv):
            wp_north, wp_east = -wp_east, wp_north
    elif command == "L":
        mv = (value % 360) // 90
        for _ in range(mv):
            wp_north, wp_east = wp_east, -wp_north
    elif command == "F":
        north += wp_north * value
        east += wp_east * value
    print(f"---> [north={north}, east={east}, wp_north={wp_north}, wp_east={wp_east}]")

print(north, east)
print(abs(north) + abs(east))
