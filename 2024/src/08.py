from typing import Tuple, Set
import itertools
from collections import defaultdict

with open("inputs/08.in")  as f:
    input = f.read().strip().split("\n")

maxRow = maxCol = 0
antennas = defaultdict(list)
for i,row in enumerate(input):
    maxRow = i
    for j,col in enumerate(row):
        maxCol = j
        if col != ".":
            antennas[col].append((i,j))

def in_map(pos: Tuple[int, int]) -> bool:
    return pos[0]>=0 and pos[0]<=maxRow and pos[1]>=0 and pos[1]<=maxCol

def get_antinodes(pair: Tuple[Tuple[int, int]], p2=False) -> Set[Tuple[int, int]]:
    l1,l2 = pair
    dr,dc = l1[0]-l2[0], l1[1]-l2[1]
    antinodes = set()
    if not p2:
        for a in [(l1[0]+dr, l1[1]+dc), (l2[0]-dr, l2[1]-dc)]:
            if in_map(a):
                antinodes.add(a)
        return antinodes
    for dir in [1, -1]:
        acc = [l1[0], l1[1]]
        while in_map(acc):
            antinodes.add(tuple(acc))
            acc[0] = sum([acc[0], dir*dr])
            acc[1] = sum([acc[1], dir*dc])
    return antinodes


a1 = set()
a2 = set()
for _, pos in antennas.items():
    pairs = itertools.combinations(pos, 2)
    for p in pairs:
        antinodes_p1 = get_antinodes(p)
        antinodes_p2 = get_antinodes(p, True)
        for a in antinodes_p1:
            a1.add(a)
        for a in antinodes_p2:
            a2.add(a)

p1 = len(a1)
p2 = len(a2)

print(f"part1: {p1}")
print(f"part2: {p2}")
