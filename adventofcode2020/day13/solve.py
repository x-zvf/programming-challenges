#!/bin/env python3

import math
with open("input") as f:
    lines = f.readlines()

time = int(lines[0])
busses = [int(x) for x in filter(lambda a: "x" not in a, lines[1].split(","))]

minwait = float("inf")
minbus = None
for bus in busses:
    wait = bus * math.ceil(time/bus) - time
    if wait < minwait:
        minwait = wait
        minbus = bus

print(minbus*minwait)

bus_time = {}

for i, bus in enumerate(lines[1].split(",")):
    if bus == "x":
        continue
    bus = int(bus)
    bus_time[bus] = -i % bus

time_list = list(reversed(sorted(list(bus_time.items()))))
print(time_list)

result = time_list[0][1] # take largest time as base
increment = time_list[0][0]

for bus,b_time in time_list[1:]:
    print(bus, b_time, result, increment)
    while result % bus != bus_time[bus]:
        result += increment
    increment *= bus

print(result)

