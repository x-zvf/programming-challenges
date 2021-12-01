#!/bin/env python3

with open("input") as f:
    lines = f.readlines()

bags = {}

for line in lines:
    bag, contains_s = line.split(" bags contain ")
    bag = bag.strip()
    contains_s = contains_s.strip()[:-1]
    contains = []
    if contains_s != "no other bags":
        for other in contains_s.split(","):
            words = other.split()
            amount = int(words[0])
            colour = " ".join(words[1:-1])
            contains.append((amount, colour))
    bags[bag] = contains

#print(bags)

def cancontain(who, what):
    for amount, color in bags[who]:
        if amount > 0 and color == what:
            return True
    return False

def search(options, forwhat):
    result = []
    #print(f"searching for {forwhat} in {options}: ", end="")
    for option in options:
        if not cancontain(option, forwhat):
            continue
        #print(f"{option} can contain {forwhat}")
        new_options = list(options)
        new_options.remove(option)
        result.append(option)
        for x in search(new_options, option):
            result.append(x)
    if result == []:
        #print("none found")
        pass
    return result

print(len(set(search(list(bags.keys()), "shiny gold"))))




def requires(what):
    what_requires = bags[what]
    total = 0
    for count, item in what_requires:
        total += count
        total += count*requires(item)
    return total

print(f"shiny gold requires {requires('shiny gold')} bags")

