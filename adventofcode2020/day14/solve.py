#!/bin/env python3

with open("input") as f:
    lines = f.readlines()

mem = {}
mask = ""

def applymask(addr, value):
    value &= 0xfffffffff # Make sure all numbers are not larger than 36 bits
    for i, c in enumerate(reversed(mask)):
        if c == "X":
            continue
        if c == "1":
            value |= (1 << i)
            continue
        if c == "0":
            value &= (0xfffffffff ^ (1 << i))
    mem[addr] = value

for line in lines:
    op, value = line.strip().split(" = ")
    if op == "mask":
        mask = value
        continue
    else:
        addr = int(op[4:-1])
        value = int(value)
        applymask(addr, value)

res = 0
for addr, value in mem.items():
    res += value

print(res)

print("================== PART2 ===============")

def getaddresses(mask, address):
    addresses = set()
    x_count = 0
    for c in mask:
        if c == "X":
            x_count += 1
    for xs in range(2**x_count):
        addr = address
        xn = 0
        #print(f"[xs={bin(xs)}, addr={bin(addr)}]", end="")
        for i, c in enumerate(reversed(mask)):
            if c == "X":
                addr ^= int(bool(xs & (1<<xn))) << i
                xn+=1
            if c == "0":
                continue
            if c == "1":
                addr |= 1 << i
        #print(f"==> {bin(addr)}")
        addresses.add(addr)
    return addresses




mem = {}
mask = ""
for line in lines:
    op, value = line.strip().split(" = ")
    if op == "mask":
        mask = value
        continue
    else:
        orig_addr = int(op[4:-1])
        value = int(value)
        for a in getaddresses(mask, orig_addr):
            mem[a] = value


res = 0
for addr, value in mem.items():
    res += value

print(res)


