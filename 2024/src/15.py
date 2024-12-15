from typing import Dict, Tuple
from dataclasses import dataclass,field

@dataclass
class Scene:
    grid: Dict[Tuple[int, int], str]
    current: Tuple[int, int]
    max_row: int
    max_col: int
    moves: str
    dirs: Dict[str, Tuple[int, int]] = field(default_factory=lambda: {
        "^": (-1, 0),
        ">": (0, 1),
        "v": (1, 0),
        "<": (0, -1),
    })

    @classmethod
    def from_str(self, mapstr: str, moves: str , part2=False) -> "Scene":
        grid = dict()
        current = None
        max_row = max_col = 0
        for r,line in enumerate(mapstr.split("\n")):
            max_row = r
            for c,s in enumerate(line):
                if s == ".": # empty
                    continue
                if s == "@":
                    current = (r, c)
                    if part2:
                        current = (r, 2*c)
                if not part2:
                    max_col = c
                    grid[(r,c)] = s
                else:
                    max_col = 2*c+1
                    if s == "#":
                        grid[(r,2*c)] = s
                        grid[(r,2*c+1)] = s
                    if s == "O":
                        grid[(r,2*c)] = "["
                        grid[(r,2*c+1)] = "]"
                    if s == "@":
                        grid[(r,2*c)] = "@"

        return Scene(grid=grid, current=current, max_row=max_row, max_col=max_col, moves=moves.replace("\n", ""))

    def show(self) -> str:
        rows = []
        for i in range(self.max_row+1):
            line = ""
            for j in range(self.max_col+1):
                if self.grid.get((i,j)):
                    line += self.grid.get((i,j))
                else:
                    line += "."
            rows.append(line)
        return "\n".join(rows)

    def is_empty(self, pos: Tuple[int, int]) -> bool:
        return self.grid.get(pos) is None

    def is_wall(self, pos: Tuple[int, int]) -> bool:
        return self.grid.get(pos) == "#"

    def is_box(self, pos: Tuple[int, int], part2: bool =False) -> bool:
        if not part2:
            return self.grid.get(pos) == "O"
        return self.grid.get(pos) == "[" or self.grid.get(pos) == "]"

    def move_from_to(self, pos: Tuple[int, int], new_pos: Tuple[int, int]):
        assert self.grid.get(new_pos) is None
        s = self.grid.get(pos)
        assert s is not None
        del self.grid[pos]
        self.grid[new_pos] = s
        if s == "@":
            self.current = new_pos

    def run(self, part2=False, show=False):
        for i,m in enumerate(self.moves):
            self.tick(m, part2)
            if show:
                print(f"STEP: {i+1}")
                print(self.show())
                print()

    def tick(self, move: str, part2=False):
        dr,dc = self.dirs[move]
        cr,cc = self.current
        next = (cr+dr, cc+dc)
        if self.is_wall(next): # wall, stay put
            return
        if not self.grid.get(next): # empty, can move robot
            self.move_from_to(self.current, next)
        if self.is_box(next, part2): # box, check if we can push
            if not part2 or not move in ["^", "v"]:
                n = next
                while not self.is_empty(n) and not self.is_wall(n):
                    nr,nc = n
                    n = (nr+dr, nc+dc)
                if n == next or self.grid.get(n) is not None: # no available spot, abort
                    return
                else:
                    # move the column from the end
                    while n != next:
                        nr,nc = n
                        to_move = (nr+dr*-1, nc+dc*-1) # opposite direction
                        self.move_from_to(to_move, n)
                        n = to_move
                    self.move_from_to(self.current, next)
            else:
                # need to handle double col up/down
                # move in ["^", "v"]
                # which side of the box are we in ?
                if self.grid.get(next) == "[":
                    to_move = set([next, (next[0], next[1]+1)])
                else:
                    to_move = set([next, (next[0], next[1]-1)])

                layers_to_move = [to_move] # keep track of all layers of boxes to move
                while (not all(self.is_empty(n) for n in to_move) and
                        not any(self.is_wall(n) for n in to_move)):
                    new_to_move = set()
                    for n in to_move:
                        nr,nc = n
                        new_n = (nr+dr, nc+dc)
                        # check if aligned with another box or if 2 boxes
                        if self.grid.get(new_n) == self.grid.get(n):
                            # aligned with another box
                            new_to_move.add(new_n)
                        elif self.grid.get(new_n) in ["[", "]"]:
                            # in collision with another swifted box
                            if self.grid.get(new_n) == "[":
                                new_to_move.add(new_n)
                                new_to_move.add((new_n[0], new_n[1]+1))
                            elif self.grid.get(new_n) == "]":
                                new_to_move.add(new_n)
                                new_to_move.add((new_n[0], new_n[1]-1))
                        else: # empty, add if no collision
                            new_to_move.add(new_n)
                    if (not all(self.is_empty(n) for n in new_to_move) and
                        not any(self.is_wall(n) for n in to_move)):
                        # cleanup empty spaces that cannot be moved
                        cleaned_new_to_move = set()
                        for n in new_to_move:
                            if not self.is_empty(n):
                                cleaned_new_to_move.add(n)
                        new_to_move= cleaned_new_to_move

                    to_move = new_to_move
                    layers_to_move.append(to_move)

                if (to_move == set([next, (next[0], next[1]+1)]) or
                    to_move == set([next, (next[0], next[1]-1)]) or
                    not all(self.is_empty(n) for n in to_move)): # no available spot, abort
                    return
                else:
                    # move the columns from the end
                    layers_to_move = layers_to_move[::-1] # revert order and remove last empty row
                    layers_to_move = layers_to_move[1:] # revert order and remove last empty row
                    for layer in layers_to_move:
                        for n in layer:
                            nr,nc = n
                            dest = (nr+dr, nc+dc) # opposite direction
                            self.move_from_to(n, dest)
                    self.move_from_to(self.current, next)

    def gps(self) -> int:
        score = 0
        for k,v in self.grid.items():
            if v in ["O", "["]:
                score += (100*k[0]+k[1])
        return score


with open("inputs/15.in")  as f:
    input = f.read().strip().split("\n\n")

scene = Scene.from_str(*input)
scene.run()
p1 = scene.gps()

scene = Scene.from_str(*input, True)
scene.run(True)
p2 = scene.gps()

print(f"part1: {p1}")
print(f"part2: {p2}")
