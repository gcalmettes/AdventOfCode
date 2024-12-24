import itertools
from collections import defaultdict

with open("inputs/23.in")  as f:
    input = f.read().strip()

connections = defaultdict(set)
for line in input.split("\n"):
    c1, c2 = line.split('-')
    connections[c1].add(c2)
    connections[c2].add(c1)

groups = set()
stack = [(comp, frozenset({comp})) for comp in connections]
while stack:
    comp, group = stack.pop()
    for conn_comp in connections[comp]-group:
        if connections[conn_comp] >= group:
            new_group = group|{conn_comp}
            if new_group not in groups:
                groups.add(new_group)
                stack.append((conn_comp, new_group))

p1 = sum(len(group) == 3 and any(comp[0] == 't' for comp in group) for group in groups)
p2 = ','.join(sorted(max(groups, key=len)))

print(f"part1: {p1}")
print(f"part2: {p2}")
