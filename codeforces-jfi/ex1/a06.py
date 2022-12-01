from sys import stdin
import math

lines = stdin.readlines()
n, q = [int(x) for x in lines[0].strip().split()]
elements = [int(x) for i,x in enumerate(lines[1].strip().split())]
_q = [line.strip().split() for line in lines[2:]]
queries = []
for l,r in _q:
    queries.append([int(l),int(r)])


sol =[]
for _ in range(n+1):
    sol.append(0)
#sol = [0 for _ in range(n+1)]
for l,r in queries:
    sol[l-1] += 1
    sol[r] -= 1

for i in range(1,n):
    sol[i] += sol[i-1]

elements.sort()
#sol = list(filter(lambda x: x>0, sol))

lt = []
gt = []
for x in sol:
    if x < 0:
        lt.append(x)
    else:
        gt.append(x)

sol = sorted(gt)
for x in sorted(lt, reverse=True):
    sol.append(x)

#sol.sort()

ans = 0
for i in range(n):
    ans += abs(elements[i] * sol[i])


print(ans)