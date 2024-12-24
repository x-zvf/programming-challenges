#!/usr/bin/env python
import random
import sys

from z3 import *

inp = open(sys.argv[1], "r").read()
initl, connl = inp.split("\n\n", 1)
init = {x.split(": ")[0]: x.split(": ")[1] == "1" for x in initl.splitlines()}

conn = []
for l in connl.splitlines():
    a, op, b, _, r = l.split(" ")
    conn += [(a, b, op, r)]


ops = {"AND": And, "OR": Or, "XOR": Xor}

# Collect all variable names
allvars = set(init.keys())
for a, b, op, r in conn:
    allvars.update([a, b, r])


found = False


def ubrn():
    return int("".join(random.choices("01", k=45)), 2)


poss = set()
while True:
    print("============== iter  =================")
    if len(poss) == 1:
        print("============== Found =================")
        s = poss.pop()
        print(",".join(s))
        break

    X = ubrn()
    Y = ubrn()
    Z = X + Y

    s = Solver()
    for i in range(45):
        s.add(Bool(f"x{i:02}") == (X & (1 << i) != 0))
        s.add(Bool(f"y{i:02}") == (Y & (1 << i) != 0))
        s.add(Bool(f"z{i:02}") == (Z & (1 << i) != 0))
    s.add(Bool("z45") == (Z & (1 << 45) != 0))

    if len(poss) > 0:
        for c in next(iter(poss)):
            if all(c in x for x in poss):
                print(f"{c} must be swapped")
                s.add(Bool(f"ignore_{c}") == True)

    def getN(prefix):
        vs = list(sorted(filter(lambda v: v.startswith(prefix), allvars)))
        return Sum([If(Bool(y), 2**i, 0) for i, y in enumerate(vs)])

    for a, b, op, r in conn:
        s.add(Or(Bool(r) == ops[op](Bool(a), Bool(b)), Bool(f"ignore_{r}")))

    s.add(Sum([If(Bool(f"ignore_{r}"), 1, 0) for _, _, _, r in conn]) == 8)

    # past iterations, ...
    s.add(Bool("ignore_jgb") == True)
    s.add(Bool("ignore_rkf") == True)
    s.add(Bool("ignore_rrs") == True)
    s.add(Bool("ignore_rvc") == True)
    s.add(Bool("ignore_vcg") == True)
    s.add(Bool("ignore_z09") == True)
    s.add(Bool("ignore_z20") == True)
    s.add(Bool("ignore_z24") == True)

    sols = set()
    while str(s.check()) == "sat":
        m = s.model()
        ignored = tuple(
            sorted([r for _, _, _, r in conn if m[Bool(f"ignore_{r}")] == True])
        )
        sols.add(ignored)
        print(".", end="", flush=True)

        s.add(Not(And(*[Bool(f"ignore_{r}") for r in ignored])))
    if len(poss) == 0:
        poss = sols
    else:
        poss = poss.intersection(sols)
    print(f"\nDown to {len(poss)} possibilities: ", poss)
