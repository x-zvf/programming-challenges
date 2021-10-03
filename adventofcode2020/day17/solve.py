#!/bin/env python3

with open("input") as f:
    lines = f.readlines()


active_cubes = set()

for y, line in enumerate(lines):
    for x, c in enumerate(line):
        if c == "#":
            active_cubes.add((x,y,0))

def get_sim_bounds(a_cubes):
    min_x=min_y=min_z=max_x=max_y=max_z = 0
    for cube in a_cubes:
        if cube[0] < min_x: min_x = cube[0]
        if cube[1] < min_y: min_y = cube[1]
        if cube[2] < min_z: min_z = cube[2]
        if cube[0] > max_x: max_x = cube[0]
        if cube[1] > max_y: max_y = cube[1]
        if cube[2] > max_z: max_z = cube[2]
    return min_x-1,min_y-1,min_z-1,max_x+1,max_y+1,max_z+1

def a_neighbours(a_cubes, x, y, z):
    count = 0
    for xi in [-1,0,1]:
        for yi in [-1,0,1]:
            for zi in [-1,0,1]:
                if xi == 0 and yi == 0 and zi == 0:
                    continue
                if (x+xi,y+yi,z+zi) in a_cubes:
                    count += 1
    return count

def cycle(a_cubes):
    n_cubes = set(a_cubes)
    min_x,min_y,min_z,max_x,max_y,max_z = get_sim_bounds(a_cubes)
    for x in range(min_x, max_x+1):
        for y in range(min_y, max_y+1):
            for z in range(min_z, max_z+1):
                a_n = a_neighbours(a_cubes, x, y, z)
                if ((x,y,z) in a_cubes) and (a_n not in (2,3)):
                    n_cubes.remove((x,y,z))
                elif ((x,y,z) not in a_cubes) and a_n == 3:
                    n_cubes.add((x,y,z))
    return n_cubes

c_cubes = set(active_cubes)
for _ in range(6):
    c_cubes = cycle(c_cubes)

print(len(c_cubes))



print("================= PART 2 ===================")

active_cubes = set()

for y, line in enumerate(lines):
    for x, c in enumerate(line):
        if c == "#":
            active_cubes.add((x,y,0,0))

def get_sim_bounds4(a_cubes):
    min_x=min_y=min_z=min_w=max_x=max_y=max_z=max_w = 0
    for cube in a_cubes:
        if cube[0] < min_x: min_x = cube[0]
        if cube[1] < min_y: min_y = cube[1]
        if cube[2] < min_z: min_z = cube[2]
        if cube[3] < min_w: min_w = cube[3]
        if cube[0] > max_x: max_x = cube[0]
        if cube[1] > max_y: max_y = cube[1]
        if cube[2] > max_z: max_z = cube[2]
        if cube[3] > max_w: max_w = cube[3]
    return min_x-1,min_y-1,min_z-1,min_w-1,max_x+1,max_y+1,max_z+1,max_w+1

def a_neighbours4(a_cubes, x, y, z, w):
    count = 0
    for xi in [-1,0,1]:
        for yi in [-1,0,1]:
            for zi in [-1,0,1]:
                for wi in [-1,0,1]:
                    if xi == 0 and yi == 0 and zi == 0 and wi == 0:
                        continue
                    if (x+xi,y+yi,z+zi, w+wi) in a_cubes:
                        count += 1
    return count

def cycle4(a_cubes):
    n_cubes = set(a_cubes)
    min_x,min_y,min_z,min_w,max_x,max_y,max_z,max_w = get_sim_bounds4(a_cubes)
    for x in range(min_x, max_x+1):
        for y in range(min_y, max_y+1):
            for z in range(min_z, max_z+1):
                for w in range(min_w, max_w+1):
                    a_n = a_neighbours4(a_cubes, x, y, z, w)
                    if ((x,y,z,w) in a_cubes) and (a_n not in (2,3)):
                        n_cubes.remove((x,y,z,w))
                    elif ((x,y,z,w) not in a_cubes) and a_n == 3:
                        n_cubes.add((x,y,z,w))
    return n_cubes

c_cubes = set(active_cubes)
for _ in range(6):
    c_cubes = cycle4(c_cubes)

print(len(c_cubes))

