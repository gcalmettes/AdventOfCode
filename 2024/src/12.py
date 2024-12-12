from typing import List, Tuple
from collections import deque, defaultdict

with open("inputs/12.in")  as f:
    input = f.read().strip().split("\n")

GRID = dict()
maxRow = maxCol = 0
for i,row in enumerate(input):
    maxRow = i
    for j,col in enumerate(row):
        maxCol = j
        GRID[(i, j)]=col

def crawl(pos: Tuple[int, int]) -> List[Tuple[int, int]]:
    plant_type = GRID.get(pos)
    area = []
    seen = set()
    queue = deque([pos])
    while len(queue) > 0:
        p = queue.pop()
        if p not in seen and GRID.get(p) and GRID.get(p) == plant_type:
            seen.add(p)
            area.append(p)
            # crawl neighbors
            for dr, dc in [[-1, 0], [0, 1], [1, 0], [0, -1]]:
                queue.append((p[0]+dr, p[1]+dc))
    return area

def get_tile_segments(region: List[Tuple[int, int]]) -> int:
    segments = defaultdict(int)
    for tile in region:
        r,c = tile
        up = (r, c, r, c+1)
        down = (r+1, c, r+1, c+1)
        left = (r, c, r+1, c)
        right = (r, c+1, r+1, c+1)
        for segment in [up, down, right, left]:
            segments[segment] +=1
    return segments

def get_perimeter(region: List[Tuple[int, int]]) -> int:
    segments = get_tile_segments(region)
    return len([k for k,v in segments.items() if v==1])

def get_sides(region: List[Tuple[int, int]]) -> int:
    segments = get_tile_segments(region)
    initial_segments = [s for s in segments]
    # keep only exterior segments
    segments = set(k for k,v in segments.items() if v==1)
    sides = 0
    current =  (-1, -1, -1, -1) # just to declare the variable
    while len(segments) > 0:
        if len(segments) > 0 :
            current =  list(segments)[0]
        queue = deque([current])
        while len(queue) > 0:
            current = queue.pop()
            segments.remove(current)
            rs,cs,re,ce = current
            # determine if current is vertical or horizontal
            is_vert = cs == ce
            if is_vert:
                # above/below
                above_or_left = (rs-1, cs, re-1, ce)
                above_or_left_perpendicular = [
                    (rs, cs-1, rs, cs),
                    (rs, cs, rs, cs+1)
                ]
                below_or_right = (rs+1, cs, re+1, ce)
                below_or_right_perpendicular = [
                    (re, cs-1, re, cs),
                    (re, cs, re, cs+1)
                ]
            else:
                # left/right
                above_or_left = (rs, cs-1, re, ce-1)
                above_or_left_perpendicular = [
                    (rs-1, cs, rs, cs),
                    (rs, cs, rs+1, cs)
                ]
                below_or_right = (rs, cs+1, re, ce+1)
                below_or_right_perpendicular = [
                    (rs-1, ce, rs, ce),
                    (rs, ce, rs+1, ce)
                ]
            # we do not include above/left or below/right if they are corners with other zones
            if above_or_left in segments and not all([p in initial_segments for p in above_or_left_perpendicular]):
               queue.append(above_or_left)
            if below_or_right in segments and not all([p in initial_segments for p in below_or_right_perpendicular]):
                queue.append(below_or_right)
        sides += 1
    return sides

p1 = p2 = 0
visited = set()
regions = defaultdict(list)
for i in range(maxRow+1):
    for j in range(maxCol+1):
        if (i,j) not in visited:
            region = crawl((i, j))
            for p in region:
                visited.add(p)
            regions[GRID.get((i, j))].append(region)
            p1 += len(region)*get_perimeter(region)
            p2 += len(region)*get_sides(region)

print(f"part1: {p1}")
print(f"part2: {p2}")
