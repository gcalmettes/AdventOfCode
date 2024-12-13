import re
from typing import Tuple
from dataclasses import dataclass

with open("inputs/13.in")  as f:
    input = f.read().strip().split("\n\n")

COST_A = 3
COST_B = 1

# input = """Button A: X+94, Y+34
# Button B: X+22, Y+67
# Prize: X=8400, Y=5400

# Button A: X+26, Y+66
# Button B: X+67, Y+21
# Prize: X=12748, Y=12176

# Button A: X+17, Y+86
# Button B: X+84, Y+37
# Prize: X=7870, Y=6450

# Button A: X+69, Y+23
# Button B: X+27, Y+71
# Prize: X=18641, Y=10279""".split("\n\n")

@dataclass
class Button:
    X: int
    Y: int

@dataclass
class Game:
    buttons: Tuple[Button, Button]
    prize: Tuple[int, int]

    def compute_cost(self) -> int:
        A,B = self.buttons
        tX,tY = self.prize
        mini = None
        for a in range(100):
            x_from_A = a * A.X
            y_from_A = a * A.Y
            for b in range(100):
                x_from_B = b * B.X
                y_from_B = b * B.Y
                if x_from_A + x_from_B == tX and y_from_A + y_from_B == tY:
                    cost = 3*a + b
                    if mini is None:
                       mini = cost
                    elif cost < mini:
                        mini = cost
        return mini if mini else 0

    def solve(self) -> int:
        A,B = self.buttons
        tX,tY = self.prize

        # Need to solve for 2 eq
        # a * Ax + b * Bx = tx
        # a * Ay + b * By == ty
        a = (tY * B.X - tX * B.Y) / (A.Y * B.X - A.X * B.Y)
        b = -((tY * A.X - tX * A.Y) / (A.Y * B.X - A.X * B.Y))

        return int(a * 3 + b) if a.is_integer() and b.is_integer() else 0

def parse(block: str) -> Game:
    pattern = r'X[+-=](\d+),\s*Y[+-=](\d+)'
    matches = re.findall(pattern, block)
    game: Game = None
    buttons = []
    for i, match in enumerate(matches):
        x_value, y_value = match
        if i < 2:
            buttons.append(Button(X=int(x_value), Y=int(y_value)))
        else:
            game = Game(buttons=buttons, prize=(int(x_value), int(y_value)))
    return game

games = [parse(block) for block in input]

p1 = p2 = 0
for g in games:
    p1 += g.compute_cost()

    g.prize = (g.prize[0]+10000000000000, g.prize[1]+10000000000000)
    p2 += g.solve()


print(f"part1: {p1}")
print(f"part2: {p2}")
