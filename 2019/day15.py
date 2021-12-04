"""
https://adventofcode.com/2019/day/15
"""

from typing import Tuple, List, NamedTuple
from intcodeComputer import IntcodeComputer
from collections import defaultdict
from enum import Enum

class Direction:
    NORTH: int=1
    SOUTH: int=2
    WEST: int=3
    EAST: int=4


def find_oxygen(program):
    paths = []
# def find_oxygen(program):
#     # initialize space with droid position
#     space = defaultdict(int, {(0, 0): '.'})
#     droid = IntcodeComputer(program).run()
#     steps = 0
#     current_spot = 1

#     while current_spot != 2:
#         while current_spot != 1 or current_spot != 2:



#     droid.add_input(Direction.WEST)


with open('day15_input.txt') as f:
    program = [int(x) for x in f.read().strip().split(',')]




# print(next(droid.run()))

print(Direction())