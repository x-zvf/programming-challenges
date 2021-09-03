import itertools
from sys import stdin, stdout

line = stdin.readline()

n = int(line.strip())

a = [0 for _ in range(n+2)]
ans = [0 for _ in range(n+2)]

for i in range(2, n+1):
    print("?", 1, i)
    stdout.flush()
    a[i] = int(stdin.readline().strip())

for i in range(3, n+1):
    ans[i] = a[i] - a[i-1]

print("?", 2, 3)
stdout.flush()

foo = int(stdin.readline().strip())
ans[1] = a[3] - foo;  # n1 + n2 + n3 - (n2 + n3)
ans[2] = a[2] - ans[1]; # n1 + n2 - n1
print("! ", end="")
for i in range(1, n+1):
    print(ans[i], end=" ")
stdout.flush()