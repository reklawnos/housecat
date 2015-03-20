n = 10000000

s = 1
for i in range(1, n + 1):
    s = (s * i) % 100000

print s
