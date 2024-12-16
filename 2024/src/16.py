from collections import deque, defaultdict

with open("inputs/16.in")  as f:
    input = f.read().strip()

maze = set()
start = end = None
for r,line in enumerate(input.split("\n")):
    for c,s in enumerate(line):
        if s == "#":
            maze.add((r, c))
        if s == "S":
            start = (r, c)
        if s == "E":
            end = (r, c)

DIRS = {
    0: (-1, 0), # North
    1: (0, 1), # East
    2: (1, 0), # South
    3:(0, -1), # West
}

def possible_neighbors(pos):
    possible = []
    for dir, (dr,dc) in DIRS.items():
        if (pos[0]+dr, pos[1]+dc) not in maze:
            possible.append(((pos[0]+dr, pos[1]+dc), dir))
    return possible

# start facing East
start_dir = 1
p1 = 0

END = defaultdict(list)
SCORES = dict()
SEEN = set()
Q = deque([(*start, start_dir, 0, [start])]) # (r, c, dir, score, path_taken)
while Q:
    r,c,dir,score,path = Q.popleft()
    if (r,c) == end:
        END[score].append(path)
        continue
    if (r,c,dir) in SEEN:
        if SCORES[(r,c, dir)] < score:
            # abort, already same position with lower score
            continue
        else:
            SCORES[(r,c, dir)] = score
    else:
        SEEN.add((r, c, dir))
        SCORES[(r,c, dir)] = score
    possible_next = possible_neighbors((r, c))
    for p,d in possible_next:
        if d == dir:
            Q.append((*p, d, score+1, [*path, p]))
        elif abs(dir-d)%2==1:
            # 90 deg turn
            Q.append((*p, d, score+1+1000, [*path, p]))

p1 = min(END.keys())

tiles = set(sum(END[p1], []))
p2 = len(tiles)

print(f"part1: {p1}")
print(f"part2: {p2}")
