with open("inputs/25.in")  as f:
    input = f.read().strip()

def parse_schematic(lines):
    heights = [0] * 5
    for i,line in enumerate(lines):
        if i == 0:
            # first line is always filled
            continue
        for j,c in enumerate(line):
            if c == "." and lines[i-1][j] == "#":
                heights[j] = i-1
    return heights

keys = []
locks = []

for block in input.split("\n\n"):
    lines = block.split("\n")
    is_key = False
    if lines[0] == ".....": # this is a key
        is_key = True
        lines = lines[::-1]
    heights = parse_schematic(lines)
    if is_key:
        keys.append(heights)
    else:
        locks.append(heights)

p1 = 0
for key in keys:
    for lock in locks:
        fit = True
        for k,l in zip(key,lock):
            if k+l > 5:
                fit = False
        if fit:
            p1 += 1


print(f"part1: {p1}")
