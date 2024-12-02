from collections import Counter, defaultdict

with open("inputs/01.in")  as f:
    lines = f.readlines()

LEFT = []
RIGHT = []
for line in lines:
    l,r = [int(n) for n in line.split()]
    LEFT.append(l)
    RIGHT.append(r)

LEFT.sort()
RIGHT.sort()

p1, p2 = 0, 0
c = Counter(RIGHT)

for l,r in zip(LEFT, RIGHT):
    p1 += abs(l-r)
    p2 += c.get(l, 0)*l

print(f"part1: {p1}")
print(f"part2: {p2}")
