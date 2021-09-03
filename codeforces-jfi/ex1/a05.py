import itertools
from sys import stdin
import math

lines = stdin.readlines()
n, w = [int(x) for x in lines[0].strip().split()]
cups = [(int(x),i, 0) for i,x in enumerate(lines[1].strip().split())]

scups = sorted(cups, reverse=True)

for i in range(n):
    volume, idx, x = scups[i]
    mv = math.ceil(volume / 2.0)
    scups[i] = (volume, idx, x+mv)
    w -= mv

if w < 0:
    print(-1)
else:
    idx = 0
    while(w > 0):
        v,i,x = scups[idx]
        if (x+1) <= v:
            scups[idx] = (v, i, x+1)
            w -= 1
        idx += 1
        if idx >= n:
            idx = 0
    for v, i, x in sorted(scups, key=lambda z: z[1]):
        print(x, end=" ")

#
#    idx = 0
#    while(w > 0):
#        w -= 1
#        v,i,x = scups[idx]
#        scups[idx] = (v, i, x+1)
#        idx += 1
#        if idx >= n:
#            idx = 0