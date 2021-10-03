#!/bin/env python3
numbers = [13,16,0,12,15,1]
n = 2020
n = 30000000

last_number = numbers[-1]
numbers = numbers[:-1]

last_spoken = {}
for i, number in enumerate(numbers):
    last_spoken[number] = i

for i in range(len(numbers),n-1):
    if last_number not in last_spoken:
        last_spoken[last_number] = i
        numbers.append(last_number)
        last_number = 0
    else:
        ln = last_number
        last_number = i-last_spoken[last_number]
        last_spoken[ln] = i
        numbers.append(last_number)

print(last_number)
