#!/bin/env python3
import sys
with open("input") as f:
    text = f.read()

rules_s, messages_s = text.split("\n\n")

rules = {}
for rs in rules_s.split("\n"):
    name, rd = rs.strip().split(": ")
    rules[name] = [x.split() for x in rd.split(" | ")]

messages = messages_s.split("\n")


def rule_to_regex(n):
    if rules[n][0][0][0] == '"':
        return rules[n][0][0][1]
    exp_o = []
    for option in rules[n]:
        subreg = []
        for x in option:
            if len(sys.argv) == 1:
                subreg.append(rule_to_regex(x))
                continue

            if x == "8":
                subreg.append(rule_to_regex("42")+"+")
            elif x == "11":
                subreg.append("("+rule_to_regex("42")+"(?1)?"+rule_to_regex("31")+")")
            else:
                subreg.append(rule_to_regex(x))
        exp_o.append("".join(subreg))
    if len(exp_o) == 1:
        return exp_o[0]
    return "(?:" + "|".join(exp_o) + ")"

rr = "^"+rule_to_regex('0')+"$"

valid_m = []
print(rr)
#print("I SHOULD HAVE DONE THIS IN PERL TO BEFGIN WITH")

