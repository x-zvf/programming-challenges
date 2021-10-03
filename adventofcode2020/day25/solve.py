#!/bin/env python3

mod = 20201227
door_pk = 13135480 # (nk^x)mod
card_pk = 8821721

val = 1
subj_num = 7
c_steps = 0

while val != card_pk:
    val = (val * subj_num) % mod
    c_steps += 1

val = 1
for _ in range(c_steps):
    val = (val * door_pk) % mod

print(val)
