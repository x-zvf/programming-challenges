#!/bin/env python3

with open("input") as f:
    lines = f.readlines()

part2 = False
def evil(s): # what is being asked is heracy
    _s = str(s)
    idx = 0
    bcount = 0
    i_ob = 0
    while "(" in s:
        if s[idx] == "(":
            bcount += 1
            if bcount == 1:
                i_ob = idx
        if s[idx] == ")":
            bcount -= 1
            if not bcount:
                r = evil(s[i_ob+1:idx])
                s = s.replace(s[i_ob:idx+1], str(r))
                idx = i_ob
        idx += 1
    if part2:
        to_add = s.split(" * ")
        if len(to_add) > 1:
            res = 1
            for t in to_add:
                res *= evil(t)
            return res
    v = s.split()
    res = int(v[0])
    op = ""
    for o in v[1:]:
        if o == "*":
            op = "*"
        elif o == "+":
            op = "+"
        elif op == "+":
            res += int(o)
        elif op == "*":
            res *= int(o)
    return res

print(sum(map(evil, lines)))
part2=True
print(sum(map(evil, lines)))

