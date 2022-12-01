import itertools
from sys import stdin

lines = stdin.readlines()

lines = lines[1:]
for line in lines:
    a, b = line.strip().split()
    a = int(a)
    b = int(b 
    x = False
    for p in itertools.permuations(str(a)):
        if int(p) % 60 == 0:
            print("red")
            x = True
    if not x:
        print("cyan")