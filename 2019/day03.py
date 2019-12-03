from dataclasses import dataclass
from collections import namedtuple
from typing import Set, List


@dataclass
class cursor:
    x: int=0
    y: int=0


Point = namedtuple('Point', ['x', 'y'])


def get_locations(instructions: str) -> Set:
    steps = instructions.split(',')

    locations = set()
    cur = cursor()
    for step in steps:
        direction = step[0]
        magnitude = int(step[1:])
        for _ in range(magnitude):
            if direction == 'R':
                cur.x += 1
            elif direction == 'L':
                cur.x -= 1
            elif direction == 'U':
                cur.y += 1
            elif direction == 'D':
                cur.y -= 1
            else:
                raise ValueError(f'The direction {direction} is invalid')
            locations.add(Point(x=cur.x, y=cur.y))
    return locations


def get_crossings(path1: str, path2: str) -> Set:
    loc1 = get_locations(path1)
    loc2 = get_locations(path2)
    return loc1.intersection(loc2)


def compute_manhattan(p: Point) -> int:
    return abs(p.x) + abs(p.y)


def get_min_crossing_distance(path1: str, path2: str) -> int:
    crossings = get_crossings(path1, path2)
    return min(compute_manhattan(p) for p in crossings)


def get_number_of_step_to(p: Point, instructions: str) -> int:
    steps = instructions.split(',')
    cur = cursor()

    count = 0
    for step in steps:
        direction = step[0]
        magnitude = int(step[1:])
        for _ in range(magnitude):
            count += 1
            if direction == 'R':
                cur.x += 1
            elif direction == 'L':
                cur.x -= 1
            elif direction == 'U':
                cur.y += 1
            elif direction == 'D':
                cur.y -= 1
            else:
                raise ValueError(f'The direction {direction} is invalid')
            if (cur.x, cur.y) == p:
                return count


def get_steps_to_crossing(paths: List[str]) -> int:
    crossings = get_crossings(*paths)
    steps_to_crossings = [[get_number_of_step_to(c, path) for path in paths] for c in crossings]
    return steps_to_crossings


def get_min_step_to_crossing(paths: List[str]) -> int:
    return min(sum(s) for s in get_steps_to_crossing(paths))


with open('day03_input.txt', 'r') as f:
    paths = f.readlines()

# part 1
path1 = 'R8,U5,L5,D3'
path2 = 'U7,R6,D4,L4'
assert get_min_crossing_distance(path1, path2) == 6
assert get_min_step_to_crossing([path1, path2]) == 30
path1 = 'R75,D30,R83,U83,L12,D49,R71,U7,L72'
path2 = 'U62,R66,U55,R34,D71,R55,D58,R83'
assert get_min_crossing_distance(path1, path2) == 159
assert get_min_step_to_crossing([path1, path2]) == 610
path1 = 'R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51'
path2 = 'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7'
assert get_min_crossing_distance(path1, path2) == 135
assert get_min_step_to_crossing([path1, path2]) == 410

part1 = get_min_crossing_distance(*paths)
print(f'part 1: {part1}')
part2 = get_min_step_to_crossing(paths)
print(f'part 2: {part2}')
