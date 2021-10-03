#!/bin/env python3
import itertools

with open("input", 'r') as f:
    lines = f.readlines()
    numbers = [int(x.strip()) for x in lines]

r = list(filter(lambda x: sum(x)==2020, itertools.combinations(numbers,3)))[0]
prod = 1
for x in r:
    prod *= x
print(prod)
