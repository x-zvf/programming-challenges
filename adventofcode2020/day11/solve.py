#!/bin/env python3

with open("input") as f:
    lines = f.readlines()

initial = []
for y in lines:
    initial.append(list(y.strip()))

def step(state):
    new = []
    for row in state:
        new.append(list(row))

    def isoccupied(x,y):
        if x < 0 or y < 0 or x >= len(state[0]) or y>= len(state):
            return False
        return state[y][x] == "#"

    delta = 0

    for y in range(len(state)):
        for x in range(len(state[y])):
            if state[y][x] == ".":
                continue
            neighbourcount = 0
            neighbourcount += int(isoccupied(x-1,y-1))
            neighbourcount += int(isoccupied(x-0,y-1))
            neighbourcount += int(isoccupied(x+1,y-1))
            neighbourcount += int(isoccupied(x-1,y-0))
            neighbourcount += int(isoccupied(x+1,y-0))
            neighbourcount += int(isoccupied(x-1,y+1))
            neighbourcount += int(isoccupied(x-0,y+1))
            neighbourcount += int(isoccupied(x+1,y+1))

            if neighbourcount == 0 and state[y][x] == "L":
                delta += 1
                new[y][x] = "#"
            elif neighbourcount > 3 and state[y][x] == "#":
                delta += 1
                new[y][x] = "L"

    n_occupied = 0
    for y in range(len(new)):
        for x in range(len(state[y])):
            n_occupied += int(isoccupied(x,y))

    return new, delta, n_occupied

current = list(initial)
while True:
    current, delta, occ = step(current)
    if delta == 0:
        for y in current:
            print("".join(y))
        print("============================")
        print(occ)
        break

print("============================")

def step2(state):
    new = []
    for row in state:
        new.append(list(row))

    def traverse(x,y,xinc, yinc):
        while 0<=x<len(state[0]) and 0<=y<len(state):
            x+= xinc
            y+= yinc
            if 0<=x<len(state[0]) and 0<=y<len(state) and state[y][x] == "#":
                return True
            elif 0<=x<len(state[0]) and 0<=y<len(state) and state[y][x] == "L":
                return False
        return False

    delta = 0

    for y in range(len(state)):
        for x in range(len(state[y])):
            if state[y][x] == ".":
                continue

            neighbourcount = 0
            neighbourcount += int(traverse(x,y, 1, 1))
            neighbourcount += int(traverse(x,y, 1, 0))
            neighbourcount += int(traverse(x,y, 1, -1))
            neighbourcount += int(traverse(x,y, 0, 1))
            neighbourcount += int(traverse(x,y, 0, -1))
            neighbourcount += int(traverse(x,y, -1, 1))
            neighbourcount += int(traverse(x,y, -1, 0))
            neighbourcount += int(traverse(x,y, -1, -1))

            if neighbourcount == 0 and state[y][x] == "L":
                delta += 1
                new[y][x] = "#"
            elif neighbourcount > 4 and state[y][x] == "#":
                delta += 1
                new[y][x] = "L"

    n_occupied = 0
    for y in range(len(new)):
        for x in range(len(state[y])):
            n_occupied += int(new[y][x]=="#")

    return new, delta, n_occupied


current = list(initial)
while True:
    current, delta, occ = step2(current)
    if delta == 0:
        for y in current:
            print("".join(y))
        print("============================")
        print(occ)
        break
