#!/bin/env python3

_cups = [6,8,5,9,7,4,2,1,3]
#_cups = [3,8,9,1,2,5,4,6,7]

cups = list(_cups)
length = len(cups)
lowest = min(cups)
highest = max(cups)
curr_id = 0

for i in range(100):
    #print(f"-- move {i+1} --")
    #print("cups: ", end="")
    #for it,c in enumerate(cups):
    #    if it == curr_id:
    #        print(f"({c}) ",end="")
    #    else:
    #        print(c,"",end="")

    #print()

    dest = cups[curr_id] - 1
    take = (cups+cups)[curr_id+1:curr_id+4]
    for t in take:
        cups.remove(t)
    next_c = cups[(cups.index(dest+1)+1)%len(cups)]

    #print("pick up:", str(take)[1:-1])

    while dest not in cups:
        dest -= 1
        if dest < lowest:
            dest = highest

    #print("destination:", dest)
    did = cups.index(dest)
    addr = (did + 1) % length
    for t in take:
        cups.insert(addr, t)
        addr += 1
        addr %= length
    curr_id = cups.index(next_c)
    #print()

res = []
idx = (cups.index(1)+1) % length
while idx != curr_id:
    res.append(str(cups[idx]))
    idx += 1
    idx %= length

print("".join(res))


class Cup:
    def __init__(self, v):
        self.v = v
        self.next = None
    def __str__(self):
        return str(self.v)
    def __repr__(self):
        return f"Cup(v={self.v}; next={self.next})"
    def ins(self, other):
        foo = other.next.next
        foo.next, self.next = self.next, other

cups = [Cup(x) for x in range(1000001)]
c = list(_cups)
c += list(range(len(c)+1, 1000001))
for a,b in zip(c, c[1:]):
    cups[a].next = cups[b]
cups[c[-1]].next=cups[c[0]]
curr = cups[c[0]]
largest = len(cups)-1

for i in range(10**7):
    if i % 10000 == 0:
        print(f"{i/100000}%", end="\r")
    take = curr.next
    curr.next = take.next.next.next
    values = [take.v, take.next.v, take.next.next.v, curr.v]

    dest = curr.v - 1
    if dest < 1:
        dest = largest
    while cups[dest].v in values:
        dest -= 1
        if dest < 1:
            dest = largest

    curr = curr.next
    cups[dest].ins(take)

s1 = cups[1].next.v
s2 = cups[1].next.next.v
print(s1*s2)

