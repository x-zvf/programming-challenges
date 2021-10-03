#!/bin/env python3

with open("testinput") as f:
    lines = f.readlines()

foods = []

allergens_in_foods = {}

all_ingr = set()
all_aller = set()
ingr_count = {}

for line in lines:
    ingredients, allergens = line.strip().split(" (contains")
    allergens = allergens[:-1].split(", ")
    ingredients = ingredients.split()
    for ing in ingredients:
        all_ingr.add(ing)
        ingr_count[ing] = ingr_count.get(ing,0) + 1
    foods.append((set(ingredients), set(allergens)))
    for al in allergens:
        all_aller.add(al)
        if al not in allergens_in_foods:
            allergens_in_foods[al] = []
        allergens_in_foods[al].append(set(ingredients))

a_could_be = {}
for a, fs in allergens_in_foods.items():
    a_could_be[a] = set.intersection(*fs)

allerg_ingr = []
for al in all_aller:
    pos = set()
    for ingr, allerg in foods:
        if al in allerg:
            if pos:
                pos = pos.intersection(ingr)
            else:
                pos = set(ingr)
    allerg_ingr.append((al,pos))

no_a_ingr = set(all_ingr)
for allerg, ingr in allerg_ingr:
    no_a_ingr -= ingr

count = 0
for ingr, allerg in foods:
    count += len(no_a_ingr.intersection(ingr))

print(no_a_ingr)
print(count)


import copy

all_a = set()
all_i = set()
for i, a in foods:
    all_a = all_a.union(set(a))
    all_i = all_i.union(set(i))

# Find out all the allergens and the respective possible ingredients
a_i = []

for target in all_a:
    possible_i = set()

    for i, a in foods:
        if target in a:
            possible_i = possible_i.intersection(i) if possible_i else set(i)

    a_i.append((target, possible_i))

inert = copy.deepcopy(all_i)
for _, ingredients in a_i:
    inert -= ingredients

# Count the number of appearance of the inert ingredients
count = 0
for i, _ in foods:
    count += len(inert.intersection(i))

print(count)
