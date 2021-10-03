#!/bin/env python3

with open("input") as f:
    lines = f.readlines()

numbers = [int(x) for x in lines]

numbers.sort()
numbers.insert(0,0)
print(numbers)
numbers.append(numbers[-1]+3)

diffs = dict()
for i in range(1,4):
    diffs[i] = 0

for i in range(len(numbers)-1):
    d = numbers[i+1] - numbers[i]
    diffs[d] += 1

print(diffs)
print(diffs[1] * diffs[3])


cache = {}
cache[len(numbers)-1] = 1
def possiblilities(idx):
    res = 0
    if idx in cache:
        return cache[idx]
    for i in range(1,4):
        if idx + i >= len(numbers) or (numbers[idx+i] - numbers[idx]) > 3:
            break
        res += possiblilities(idx+i)
    cache[idx] = res
    return res

print(possiblilities(0))
