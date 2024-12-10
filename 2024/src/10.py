with open("inputs/10.in")  as f:
    input = f.read().strip().split("\n")

GRID = dict()
for i,row in enumerate(input):
    for j,col in enumerate(row):
        GRID[(i, j)]=int(col)

def crawl(pos, summit=[]):
    v = GRID.get(pos)
    if v is None:
        return []
    if v == 9:
        summit.append(pos)
        return summit
    for dr,dc in [(-1,0), (0, 1), (1, 0), (0, -1)]:
        next = (pos[0]+dr, pos[1]+dc)
        if GRID.get(next) == v + 1:
            summit += crawl(next, [])
    return summit

p1 = p2 =0
for k,v in GRID.items():
    if v == 0:
        summits = crawl(k, [])
        if summits:
            p1 += len(set(summits))
            p2 += len(summits)
print(f"part1: {p1}")
print(f"part2: {p2}")
