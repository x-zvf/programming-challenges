from z3 import *

opt = Optimize()

s = BitVec("s", 64)

A, B, C = s, 0, 0

opt.add(s < 2**48)
opt.add(s >= 2**45)

for x in [2, 4, 1, 3, 7, 5, 4, 7, 0, 3, 1, 5, 5, 5, 3, 0]:
    B = 3 ^ (A & 7)
    B ^= A / (1 << B)
    B ^= 5
    A >>= 3
    opt.add((B % 8) == x)


opt.add(A == 0)
opt.minimize(s)

if str(opt.check()) == "sat":
    print(opt.model().eval(s))
else:
    print("unsat")
