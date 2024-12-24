#!/usr/bin/env python
import sys

from z3 import *

inp = open(sys.argv[1], "r").read()
initl, connl = inp.split("\n\n", 1)
init = {x.split(": ")[0]: x.split(": ")[1] == "1" for x in initl.splitlines()}

conn = []
for l in connl.splitlines():
    a, op, b, _, r = l.split(" ")
    conn += [(a, b, op, r)]

s = Solver()
for k, v in init.items():
    s.add(Bool(k) == v)

ops = {"AND": And, "OR": Or, "XOR": Xor}
for a, b, op, r in conn:
    s.add(Bool(r) == ops[op](Bool(a), Bool(b)))

assert str(s.check()) == "sat"
m = s.model()


allvars = set(init.keys())
for a, b, op, r in conn:
    allvars.update([a, b, r])


def getN(l):
    vs = sorted(list(filter(lambda v: v.startswith(l), allvars)))
    return sum(2 ** i if s.model()[Bool(v)] else 0 for i, v in enumerate(vs))


print("part1: ", getN("z"))
