#!/bin/env python3
import itertools

with open("input") as f:
    lines = f.read()

fields_s, ynt = lines.split("\n\nyour ticket:\n")
your_ticket, nearby_tickets_s = ynt.split("\n\nnearby tickets:\n")

your_ticket = [int(x) for x in your_ticket.strip().split(",")]

nearby_tickets = []
for nt in nearby_tickets_s.split("\n")[:-1]:
    nearby_tickets.append(tuple([int(x) for x in nt.strip().split(",")]))

fields = {}
for f in fields_s.split("\n"):
    name, ranges = f.strip().split(":")
    ranges = ranges.strip().split(" or ")
    r = []
    for a in ranges:
        r.append(tuple([int(x) for x in a.split("-")]))
    fields[name] = r


def matches_fields(number):
    matches = []
    for name, ranges in fields.items():
        match = False
        for min_n, max_n in ranges:
            if min_n <= number <= max_n:
                match = True
                break
        if match:
            matches.append(name)
            continue
    return matches

invalid_n_t = []
iv_sum = 0
for ticket in nearby_tickets:
    invalid = False
    for value in ticket:
        if not matches_fields(value):
            if not invalid:
                invalid_n_t.append(ticket)
            invalid = True
            iv_sum += value
            break


valid_t = set(nearby_tickets)-set(invalid_n_t)

print(valid_t)

field_list = list(fields.items())

_valid_fields = [[] for _ in range(len(field_list))]
for name, ranges in field_list:
    for i in range(len(field_list)):
        valid = True
        for ticket in valid_t:
            valid_f_t = False
            for min_v, max_v in ranges:
                if min_v <= ticket[i] <= max_v:
                    valid_f_t = True
                    break
            if not valid_f_t:
                valid = False
                break
        if valid:
            _valid_fields[i].append(name)

valid_fields = []
for vf in _valid_fields:
    valid_fields.append(tuple(vf))

#print(valid_fields)
print("valid_fields calculated")

cache = {}
def findvalid(a_fields, lst):
    perm = []
    if str((a_fields, lst)) in cache:
        return cache[str((a_fields,lst))]
    if len(lst) == 1:
        for x in lst[0]:
            if x not in a_fields:
                continue
            perm.append([x])
        return perm
    for x in lst[0]:
        if x not in a_fields:
            continue
        #print(f"valid x:", x)
        n_fields = set(a_fields)
        n_fields.remove(x)
        for a in findvalid(n_fields, lst[1:]):
            perm.append([x]+a)
    cache[str((a_fields,lst))] = perm
    return perm

ticket_fields = findvalid(list(fields.keys()),valid_fields)[0]
print(ticket_fields)

res = 1
for i, (tf,value) in enumerate(zip(ticket_fields, your_ticket)):
    if "departure" in tf:
        res *= value

print(res)

