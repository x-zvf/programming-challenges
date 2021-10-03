#!/bin/env python3
import itertools

with open("input", 'r') as f:
    lines = f.readlines()
    numbers = [int(x.strip()) for x in lines]

a,b = list(filter(lambda x: x[0]+x[1]==2020, itertools.combinations(numbers,2)))[0]

print(a,"*",b," = ", a*b)
