import sys
from itertools import product

inp = open(sys.argv[1], "r").read().strip().split("\n\n")
locks, keys = [], []
for e in inp:
    (locks, keys)[e.startswith(".")].append(e)


def parse(l):
    return [
        tuple(map(lambda t: sum(1 for _ in filter(lambda x: x == "#", t)) - 1, z))
        for z in map(lambda x: zip(*x.split("\n")), l)
    ]


locks, keys = parse(locks), parse(keys)
print(
    sum(
        1 if all(x + y <= 5 for x, y in zip(l, k)) else 0
        for l, k in product(locks, keys)
    )
)
