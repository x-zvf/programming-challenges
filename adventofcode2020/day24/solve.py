#!/bin/env python3

with open("input") as f:
    lines = f.readlines()

def str_to_xy(s):
    x = 0
    y = 0
    while s:
        if s[0] == "e":
            x += 2
            s = s[1:]
        elif s[0:2] == "se":
            x += 1
            y -= 1
            s = s[2:]
        elif s[0:2] == "sw":
            x -= 1
            y -= 1
            s = s[2:]
        elif s[0] == "w":
            x -= 2
            s = s[1:]
        elif s[0:2] == "nw":
            x -= 1
            y += 1
            s = s[2:]
        elif s[0:2] == "ne":
            x += 1
            y += 1
            s = s[2:]
    return x, y

coord_count = {}

for line in lines:
    coord = str_to_xy(line.strip())
    coord_count[coord] = coord_count.get(coord, 0) + 1

black = 0
for count in coord_count.values():
    black += count % 2

print(black)

# === PART2


blacks = set()
min_x = float("inf")
min_y = float("inf")
max_x = float("-inf")
max_y = float("-inf")


for coord, count in coord_count.items():
    if count % 2:
        blacks.add(coord)

def get_sym_bounds():
    min_x = float("inf")
    min_y = float("inf")
    max_x = float("-inf")
    max_y = float("-inf")
    for coord in blacks:
        if coord[0] < min_x: min_x = coord[0]
        if coord[1] < min_y: min_y = coord[1]
        if coord[0] > max_x: max_x = coord[0]
        if coord[1] > max_y: max_y = coord[1]
    return min_x-2, min_y-2, max_x+2, max_y+2

def count_neighbours(x,y):
    count = int((x+2, y) in blacks)
    count += int((x+1, y-1) in blacks)
    count += int((x-1, y-1) in blacks)
    count += int((x-2, y) in blacks)
    count += int((x-1, y+1) in blacks)
    count += int((x+1, y+1) in blacks)
    return count

for day in range(100):
    new_blacks = set(blacks)
    min_x, min_y, max_x, max_y = get_sym_bounds()
    for x in range(min_x, max_x+1):
        for y in range(min_y, max_y+1):
            nc = count_neighbours(x,y)
            if ((x,y) in blacks) and (nc == 0 or nc > 2):
                new_blacks.remove((x,y))
            elif ((x,y) not in blacks) and nc == 2:
                new_blacks.add((x,y))

    blacks = new_blacks
print(f"{day+1}: {len(blacks)}")

