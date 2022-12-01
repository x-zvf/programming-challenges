#!/bin/env python3

with open("input") as f:
    lines = f.readlines()


def evaluate(program):
    executed = set()
    ip = 0
    acc = 0
    success = False
    while ip not in executed:
        executed.add(ip)
        instr = program[ip].strip()
        opcode, value = instr.split()
        value = int(value)
        if opcode == "nop":
            ip += 1
        elif opcode == "acc":
            acc += value
            ip += 1
        elif opcode == "jmp":
            ip += value
        if ip == len(program):
            success = True
            break
    return success, acc


print(evaluate(lines)[1])

for i in range(len(lines)):
    program = list(lines)
    instr = program[i].strip()
    opcode, value = instr.split()
    if opcode == "acc":
        continue
    if opcode == "nop":
        program[i] = f"jmp {value}"
    elif opcode == "jmp":
        program[i] = f"nop {value}"
    success, acc = evaluate(program)
    if success:
        print(f"success after changing instruction nr. {i}; acc={acc}")
        break

