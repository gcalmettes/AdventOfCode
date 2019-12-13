"""
https://adventofcode.com/2019/day/11
"""

from intcodeComputer import IntcodeComputer


from typing import List, Tuple
from dataclasses import dataclass
from collections import defaultdict, namedtuple


@dataclass
class Robot:
    x: int = 0
    y: int = 0
    cursor: int = 0

    def advance(self, instruction: int) -> Tuple[int, int]:
        facing = ['up', 'left', 'down', 'right'][self.cursor % 4]
        if instruction == 0:  # turn left
            if facing == 'up':
                self.x -= 1 # go left
            elif facing == 'left':
                self.y += 1 # go down
            elif facing == 'down':
                self.x += 1 # go right
            elif facing == 'right':
                self.y -= 1 # go up
            self.cursor += 1
        elif instruction == 1:  # turn right
            if facing == 'up':
                self.x += 1 # go right
            elif facing == 'right':
                self.y += 1 # go down
            elif facing == 'down':
                self.x -= 1 # go left
            elif facing == 'left':
                self.y -= 1 # go up
            self.cursor -= 1
        return self.x, self.y   


def paint_space(space: defaultdict) -> str:
    WIDTH = max(coord[0] for coord in space.keys()) + 1
    HEIGHT = max(coord[1] for coord in space.keys()) + 1
    result = ''
    for i in range(HEIGHT):
        for j in range(WIDTH):
            color = '#' if space[(j, i)] == 1 else ' '
            result += color
        result += '\n'
    return result
    


with open('day11_input.txt') as f:
    program = [int(x) for x in f.read().strip().split(',')]



robot = Robot()
space = defaultdict(int, {(0, 0): 0})  # robot is at (0, 0) COLOR is BLACK (0)
cpu = IntcodeComputer(program)

while True:
    try:
        color = space[(robot.x, robot.y)]
        cpu.add_input(color)
        [color, direction] = [i for i in cpu.run()]

        space[(robot.x, robot.y)] = color
        robot.advance(direction)
    except:
        break

part1 = len(space)
print(f'part 1: {part1}')



robot = Robot()
space = defaultdict(int, {(0, 0): 1})  # robot is at (0, 0) COLOR is WHITE (1)
cpu = IntcodeComputer(program)

while True:
    try:
        color = space[(robot.x, robot.y)]
        cpu.add_input(color)
        color = next(cpu.run())
        direction = next(cpu.run())

        space[(robot.x, robot.y)] = color
        robot.advance(direction)
    except:
        break

part2 = paint_space(space)
print('part 2:')
print(part2)
