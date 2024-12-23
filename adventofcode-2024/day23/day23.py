import sys

import networkx as nx

inp: any = [tuple(x.strip().split("-")) for x in open(sys.argv[1], "r").readlines()]

g = nx.Graph()
g.add_edges_from(inp)

p1 = 0
largest = []
for c in nx.enumerate_all_cliques(g):
    largest = c
    if len(c) != 3:
        continue
    if any(x.startswith("t") for x in c):
        p1 += 1

p2 = ",".join(sorted(largest))
print("part1: ", p1)
print("part2: ", p2)
