with open("inputs/04.in")  as f:
    lines = f.read().strip().split("\n")

total_len = len(lines)

grid = dict()
for j,line in enumerate(lines):
    for i, c in enumerate(line.strip()):
        grid[(i,j)] = c

def neighbors_dir(coord):
    i,j = coord
    dir = []
    #XMAS is 4 letters long
    # up
    if j>=3:
        up = [(i, j1) for j1 in range(j, j-4, -1)]
        dir.append(up)
    # down
    if j <= total_len - 4:
        down = [(i, j1) for j1 in range(j, j+4)]
        dir.append(down)
    # left
    if i >=3:
        left = [(i1, j) for i1 in range(i, i-4, -1)]
        dir.append(left)
    # right
    if i <= total_len - 4:
        right = [(i1, j) for i1 in range(i, i+4)]
        dir.append(right)
    # up/left
    if j>=3 and i >=3:
        upleft = [(i1, j1) for i1, j1 in zip(range(i, i-4, -1), range(j, j-4, -1))]
        dir.append(upleft)
    # up/right
    if j>=3 and i <= total_len - 4:
        upright = [(i1, j1) for i1, j1 in zip(range(i, i+4), range(j, j-4, -1))]
        dir.append(upright)
    # down/left
    if j <= total_len - 4 and i >=3:
        downleft = [(i1, j1) for i1, j1 in zip(range(i, i-4, -1), range(j, j+4))]
        dir.append(downleft)
    # down/right
    if j <= total_len - 4 and i <= total_len - 4:
        downright = [(i1, j1) for i1, j1 in zip(range(i, i+4), range(j, j+4))]
        dir.append(downright)
    return dir

p1 = 0

for i in range(total_len):
    for j in range(total_len):
        coord = (i, j)
        if grid[coord] == "X":
            dirs = neighbors_dir(coord)
            for d in dirs:
                grid_letters = [grid[c] for c in d]
                match = [l1==l2 for l1,l2 in zip(grid_letters, "XMAS")]
                if all(match):
                    p1 +=1

p2 = 0
for i in range(total_len):
    for j in range(total_len):
        coord = (i, j)
        if grid[coord] == "A":
            try:
                if grid[(i-1, j-1)] == "M" and grid[(i+1, j+1)] == "S":
                    if grid[(i+1, j-1)] == "S" and grid[(i-1, j+1)] == "M":
                        p2 += 1
                if grid[(i-1, j-1)] == "S" and grid[(i+1, j+1)] == "M":
                    if grid[(i+1, j-1)] == "M" and grid[(i-1, j+1)] == "S":
                        p2 += 1
                if grid[(i-1, j-1)] == "M" and grid[(i+1, j+1)] == "S":
                    if grid[(i+1, j-1)] == "M" and grid[(i-1, j+1)] == "S":
                        p2 += 1
                if grid[(i-1, j-1)] == "S" and grid[(i+1, j+1)] == "M":
                    if grid[(i+1, j-1)] == "S" and grid[(i-1, j+1)] == "M":
                        p2 += 1
            except:
                continue

print(f"part1: {p1}")
print(f"part2: {p2}")
