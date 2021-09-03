import itertools
from sys import stdin

lines = stdin.readlines()

n, walrus = lines
n = int(n.strip())

wtuple = [(int(x),i) for i, x in enumerate(walrus.strip().split())]

wtuple.sort()

mmax = -1
x = [0 for _ in range(n)]
for i in range(n):
    x[wtuple[i][1]] = max(0, mmax - wtuple[i][1])
    mmax = max(mmax, wtuple[i][1])

for a in x:
    print(a-1, end=" ")