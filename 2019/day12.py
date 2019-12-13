from dataclasses import dataclass
import re
import itertools
import math

from typing import List

@dataclass
class Asteroid:
    x: int
    y: int
    z: int
    vx: int=0
    vy: int=0
    vz: int=0

    def step(self):
        self.x += self.vx
        self.y += self.vy
        self.z += self.vz

    def energy(self):
        # potential
        pot = abs(self.x) + abs(self.y) + abs(self.z)
        # kinetic
        kin = abs(self.vx) + abs(self.vy) + abs(self.vz)
        return pot * kin


def parse_input(inputs: str) -> List[Asteroid]:
    regex = '<x=(-?\d+), y=(-?\d+), z=(-?\d+)>'
    return [
        Asteroid(*map(int, re.match(regex, line).groups())) for line in inputs.split('\n')
    ]

def apply_gravity(a1: Asteroid, a2: Asteroid) -> None:
    if a1.x > a2.x:
        a1.vx -= 1
        a2.vx += 1
    elif a1.x < a2.x:
        a1.vx += 1
        a2.vx -= 1

    if a1.y > a2.y:
        a1.vy -= 1
        a2.vy += 1
    elif a1.y < a2.y:
        a1.vy += 1
        a2.vy -= 1

    if a1.z > a2.z:
        a1.vz -= 1
        a2.vz += 1
    elif a1.z < a2.z:
        a1.vz += 1
        a2.vz -= 1



def run_part1(inputs: str, n_steps: int=100) -> int:
    asteroids = parse_input(inputs)

    for _ in range(n_steps): 
        pairs = itertools.combinations(asteroids, 2)
        for [a1, a2] in pairs:
            apply_gravity(a1, a2)
        for asteroid in asteroids:
            asteroid.step()

    return sum(a.energy() for a in asteroids)


# This function computes LCM
def lcm(a, b):
    return abs(a*b) // math.gcd(a, b)


def run_part2(inputs: str) -> int:
    asteroids = parse_input(inputs)
 
    seen_x = False
    seen_y = False
    seen_z = False

    x = set(tuple((a.x, a.vx) for a in asteroids))
    y = set(tuple((a.y, a.vy) for a in asteroids))
    z = set(tuple((a.z, a.vz) for a in asteroids))

    i = 0
    while not seen_x or not seen_y or not seen_z:
        pairs = itertools.combinations(asteroids, 2)
        for [a1, a2] in pairs:
            apply_gravity(a1, a2)
        for asteroid in asteroids:
            asteroid.step()

        x_state = (tuple((a.x, a.vx) for a in asteroids))
        y_state = (tuple((a.y, a.vy) for a in asteroids))
        z_state = (tuple((a.z, a.vz) for a in asteroids))

        if not seen_x:
            if x_state in x:
                seen_x = True
                xi = i
            else:
                x.add(x_state)

        if not seen_y:
            if y_state in y:
                seen_y = True
                yi = i
            else:
                y.add(y_state)

        if not seen_z:
            if z_state in z:
                seen_z = True
                zi = i
            else:
                z.add(z_state)  
        i+=1 

    return lcm(xi, lcm(yi, zi))


s = """<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"""
assert run_part1(s, 10) == 179


inputs = """<x=13, y=9, z=5>
<x=8, y=14, z=-2>
<x=-5, y=4, z=11>
<x=2, y=-6, z=1>"""

part1 = run_part1(inputs, 1000)
print(f'part 1: {part1}')

part2 = run_part2(inputs)
print(f'part 2: {part2}')