#!/bin/env python3

with open("input") as f:
    s = str(f.read())

groups = s.split("\n\n")

counts = []

for group in groups:
    count = 0
    for letter_c in range(26):
        letter = chr(ord("a") + letter_c)
        if letter in group:
            count += 1
    counts.append(count)

print(sum(counts))

counts_2 = []

for group in groups:
    count = 0
    for letter_c in range(26):
        letter = chr(ord("a") + letter_c)
        for person in group.split("\n"):
            if not person:
                continue
            if letter not in person:
                break
        else:
            count+= 1
    counts_2.append(count)

print(sum(counts_2))
