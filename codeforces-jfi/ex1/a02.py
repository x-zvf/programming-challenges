import itertools
from sys import stdin

lines = stdin.readlines()

lines = lines[1:]
for line in lines:
    a = line.strip()
    count = 0
    summe = 0
    zero = False
    for c in a:
        z = int(c)
        if(z % 2 == 0):
            count += 1
        if(z==0):
            zero = True
        summe += z
    if (summe % 3 == 0 and count > 1 and zero):
        print("red")
    else:
        print("cyan")