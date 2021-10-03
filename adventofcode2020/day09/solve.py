#!/bin/env python3
import itertools
preamble_len = 25

with open("input") as f:
    lines = f.readlines()

numbers = [int(x) for x in lines]

for i in range(preamble_len, len(numbers)):
    found = False
    num = numbers[i]
    for a,b in itertools.combinations(numbers[i-preamble_len:i], 2):
        if a+b == num:
            found = True
            break
    if not found:
        print(num, "not found")
        break

for i in range(len(numbers)):
    for j in range(2,len(numbers)):
        cont = numbers[i:i+j]
        s = sum(cont)
        if s == num:
            print(cont, "summed =", num)
            print(min(cont), "+", max(cont),"=", max(cont)+min(cont))
        elif s > num:
            break
