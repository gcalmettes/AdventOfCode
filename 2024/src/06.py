from collections import OrderedDict
from typing import Tuple, Set

with open("inputs/06.in")  as f:
    input = f.read()


class Grid:
    def __init__(self, obstacles: Set, guard: Tuple[int, int], current_dir: str, maxRow: int, maxCol: int):
        self.guard = guard
        self.obstacles = obstacles
        self.current_dir = current_dir
        self.DIRS = self.get_directions()
        self.DIRS_KEYS = list(self.DIRS.keys())
        self.maxRow = maxRow
        self.maxCol = maxCol

    @classmethod
    def from_input(cls, input: str):
        obstacles = set()
        guard: Tuple[int, int]
        dir: str
        maxRow = 0
        maxCol = 0
        for i,row in enumerate(input.strip().split("\n")):
            maxRow = i
            for j,col in enumerate(row):
                maxCol = j
                if col=="#":
                    obstacles.add((i,j))
                elif col!=".":
                    guard = (i, j)
                    dir = col
        return Grid(obstacles, guard, dir, maxRow, maxCol)

    @classmethod
    def get_directions(cls) -> OrderedDict:
        dirs = OrderedDict()
        for d,move in [("^", (-1,0)), (">", (0, 1)), ("v", (1, 0)), ("<", (0, -1))]:
            dirs[d] = move
        return dirs

    def next_pos(self) -> Tuple[int, int]:
        while True:
            move = self.DIRS.get(self.current_dir)
            next_pos = (self.guard[0]+move[0], self.guard[1]+move[1])
            if next_pos not in self.obstacles:
                return next_pos
            # obstacle, 90deg right turn
            dirs = self.DIRS_KEYS
            self.current_dir = dirs[(dirs.index(self.current_dir) + 1) % len(dirs)]

    def move_guard(self, pos: Tuple[int, int]) -> bool:
        self.guard = pos
        if 0<=pos[0]<=self.maxRow and 0<=pos[1]<=self.maxCol:
            return True
        return False

def run(grid) -> Tuple[int, bool]:
    count = 0
    seen = set()
    seen_with_dir = set()
    infinite=False
    while True:
        count+=1
        seen.add(grid.guard)
        pos_with_dir = (*grid.guard, grid.current_dir)
        if pos_with_dir in seen_with_dir:
            infinite = True
            break
        seen_with_dir.add((*grid.guard, grid.current_dir))
        next_pos = grid.next_pos()
        ok = grid.move_guard(next_pos)
        if not ok:
            break
    return len(seen), infinite, seen

grid = Grid.from_input(input)
start = grid.guard

p1, _, path = run(grid)

p2 = 0
for pos in path:
    if  pos != start:
        # run simulation
        grid = Grid.from_input(input)
        grid.obstacles.add(pos)
        _, infinite, _ = run(grid)
        if infinite:
            p2 += 1

print(f"part1: {p1}")
print(f"part2: {p2}")
