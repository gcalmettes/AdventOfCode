from collections import deque, defaultdict

with open("inputs/18.in")  as f:
    input = f.read().strip()


bytes = []
for line in input.split("\n"):
    x,y = line.split(",")
    x,y = int(x), int(y)
    bytes.append((x, y))

DIRS = {
    0: (0, -1), # North
    1: (1, 0), # East
    2: (0, 1), # South
    3:(-1, 0), # West
}

maxX = 70
maxY = 70
ns = 1024

def possible_neighbors(pos, grid, seen):
    possible = set()
    for dir, (dx,dy) in DIRS.items():
        p = (pos[0]+dx, pos[1]+dy)
        if 0<=p[0]<=maxX and 0<=p[1]<=maxY and p not in grid and p not in seen:
            possible.add((p, dir))
    # we want to favor going as close to the diagonal as possible
    # filter out backtrack if we can go down
    if DIRS[3] in possible and  DIRS[2] in possible:
        possible.remove(DIRS[3])
    # filter out up if we can go forward
    if DIRS[0] in possible and  DIRS[1] in possible:
        possible.remove(DIRS[0])
    return possible


start = (0, 0)
end = (maxX, maxY)

def simulate(ns):
    grid = set()
    for b in bytes[:ns]:
        grid.add((b[0], b[1]))
    END = defaultdict(list)
    SCORES = dict()
    SEEN = set()
    Q = deque([(*start, 0)]) # (r, c, score, path_taken)
    while Q:
        r,c,score = Q.popleft()
        if (r,c) == end:
            END[score].append(0)
            continue
        if (r,c) in SEEN:
            if SCORES[(r,c)] <= score:
                # abort, already same position with lower score
                continue
            else:
                SCORES[(r,c)] = score
        else:
            SEEN.add((r, c))
            SCORES[(r,c)] = score
        possible_next = possible_neighbors((r, c), grid, SEEN)
        for p,d in possible_next:
            Q.append((*p, score+1))
    if not END:
        return None
    return min(END.keys())

def binary_search(bytes, low, high):
    if high >= low:
        mid = (high + low) // 2
        # find path
        res = simulate(mid)
        if res is not None:
            # if path still available, mid is new min
            return binary_search(bytes, mid, high)
        else:
            # if not path, we want to check for previous
            previous = mid-1
            res = simulate(previous)
            if res is not None:
                return ",".join(str(b) for b in bytes[previous])
            else:
                return binary_search(bytes, low+1, mid)
    else:
        return None

p1 = simulate(ns)
p2 = binary_search(bytes, ns, len(bytes)-1)

print(f"part1: {p1}")
print(f"part2: {p2}")
