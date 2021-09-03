from sys import stdin

lines = stdin.readlines()

lines = lines[1:]
for line in lines:
    a, b = line.strip().split()
    a = int(a)
    b = int(b)
    print(a+b)