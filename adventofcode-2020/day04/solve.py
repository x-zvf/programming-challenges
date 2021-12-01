#!/bin/env python3
with open("input") as f:
    text = f.read()

passports = text.split("\n\n")

pp_keys = [[y.split(":")[0] for y in x.split()] for x in passports]

a_valid = 0
for keys in pp_keys:
    for required in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]:
        if required not in keys:
            break
    else:
        a_valid+=1

print(a_valid)


### part 2
import re
from inspect import currentframe

def has_all_fields(pp):
    keys = [x[0] for x in pp]
    for required in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]:
        if required not in keys:
            break
    else:
        return True
    return False


pp_keys = list(filter(has_all_fields,[[y.split(":") for y in x.split()] for x in passports]))

def get_linenumber():
    cf = currentframe()
    return cf.f_back.f_lineno

b_valid = 0
for kv_pairs in pp_keys:
    try:
        print(f"-----------testing: {kv_pairs}------")
        for key, value in kv_pairs:
            print(f"key: {key}, value: {value} ---> ", end="")
            if key == "byr":
                byr = int(value)
                if not 1920 <= byr <= 2002:
                    print("invalid", get_linenumber())
                    break
            elif key == "iyr":
                iyr = int(value)
                if not 2010 <= iyr <= 2020:
                    print("invalid", get_linenumber())
                    break
            elif key == "eyr":
                eyr = int(value)
                if not 2020 <= eyr <= 2030:
                    print("invalid", get_linenumber())
                    break
            elif key == "hgt":
                num = int(value[0:-2])
                if value[-2:] == "cm":
                    if not 150 <= num <= 193:
                        print("invalid", get_linenumber())
                        break
                elif value[-2:] == "in":
                    if not 59 <= num <= 76:
                        print("invalid", get_linenumber())
                        break
                else:
                    break
            elif key == "hcl":
                if not re.match(r"^#[0-9a-f]{6}$", value):
                    print("invalid", get_linenumber())
                    break
            elif key == "ecl":
                if value not in ("amb", "blu", "brn", "gry", "grn", "hzl", "oth"):
                    print("invalid", get_linenumber())
                    break
            elif key == "pid":
                if not re.match(r"^[0-9]{9}$", value):
                    print("invalid", get_linenumber())
                    break
            print("valid")
        else:
            print("VALID PASSPORT")
            b_valid += 1
    except Exception as e:
        print("invalid", get_linenumber())
        print(e)
        continue
print("\n\n=====================")
print("total valid passports:", b_valid)
