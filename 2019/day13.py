"""
https://adventofcode.com/2019/day/13
"""

from typing import Tuple, List
from intcodeComputer import IntcodeComputer

def get_loc(object: int, state: List[int]) -> Tuple[int, int]:
    idx = [i for i, val in enumerate(state) if (i+1) % 3 == 0 and val == object][0]
    return state[idx - 2], state[idx - 1]

def get_score(state: List[int]) -> int:
    try:
        idx = [i for i, val in enumerate(state) if (i+1) % 1 == -1][0]
    except:
        return False
    return state[idx + 2]

with open('day13_input.txt') as f:
    program = [int(x) for x in f.read().strip().split(',')]


cpu = IntcodeComputer(program)

part1 = len([val for i, val in enumerate(cpu.run()) if (i+1) % 3 == 0 and val == 2])
print(f'part 1: {part1}')

program_free = program[:]
program_free[0] = 2
cpu = IntcodeComputer(program_free)


game = cpu.run()
score = 0
x_paddle = 0
x_ball = 0

while True:
    # play the game
    try:
        x, y, val = next(game), next(game), next(game)
    except:
        print('End of game, thanks for playing!')
        break

    if x == -1:
        print(f'score: {val}')
        continue

    elif val == 3: # paddle
        x_paddle = x
        y_paddle = y  # this one shouldn't change

    elif val == 4: # ball
        x_ball = x
        y_ball = y

        if x_ball < x_paddle:
            # need to move the joystick left
            cpu.add_input(-1)
        elif x_ball > x_paddle:
            # need to move the joystick right
            cpu.add_input(1)
        else:
            # wait and see, don't move joystick
            cpu.add_input(0)
    else:
        continue



