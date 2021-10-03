with open("input") as f:
    lines = f.readlines()

cpassn = 0
cpassn2 = 0
for line in lines:
    req, char, pw = line.split()
    min_times, max_times = req.strip().split("-")
    char = char[0]
    pw = pw.strip()
    count = 0
    for c in pw:
        if c == char:
            count += 1
    if int(min_times) <= count <= int(max_times):
        cpassn+=1

    nc = 0
    if pw[int(min_times)-1] == char:
        nc+=1
    if pw[int(max_times)-1] == char:
        nc+=1
    if nc == 1:
        cpassn2 += 1

print(cpassn)
print(cpassn2)
