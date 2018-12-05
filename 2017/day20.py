"""
http://adventofcode.com/2017/day/20
"""

import re
from collections import Counter
from typing import NamedTuple, Tuple, List

class Particle(NamedTuple):
    pos: Tuple[int, int, int]
    vel: Tuple[int, int, int]
    acc: Tuple[int, int, int]
    id: int

def step(p: Particle) -> Particle:
    vel = (p.vel[0]+p.acc[0], p.vel[1]+p.acc[1], p.vel[2]+p.acc[2])
    pos = (p.pos[0]+vel[0], p.pos[1]+vel[1], p.pos[2]+vel[2])
    return Particle(pos, vel, p.acc, p.id)

def getDist(p: Particle) -> int:
    return sum([abs(i) for i in p.pos])

def parse(input: str, id: int) -> Particle:
    regex = "(-?[0-9]+,-?[0-9]+,-?[0-9]+)"
    pos,vel,acc = re.findall(regex, input)
    pos = tuple(int(i) for i in pos.split(","))
    vel = tuple(int(i) for i in vel.split(","))
    acc = tuple(int(i) for i in acc.split(","))
    return Particle(pos, vel, acc, id)

def removeCollided(pList: List[Particle]) -> List[Particle]:
    allPos = Counter([p.pos for p in pList])
    particles = [p for p in pList if allPos[p.pos]==1]
    return particles

if __name__ == "__main__":
    # particles = TEST_INPUT.split("\n")
    with open("day20_input.txt") as f:
        raw = f.readlines()
    
    # part 1
    particles = [parse(p, i) for i,p in enumerate(raw)]
    for i in range(1000):
        particles = [step(p) for p in particles]
    distances = list(getDist(p) for p in particles)
    print(f"particle id which stay closer to zero: {distances.index(min(distances))}")

    # part 2
    particles = [parse(p, i) for i,p in enumerate(raw)]
    print(f"starting number of particles: {len(particles)}")
    for i in range(1000):
        particles = [step(p) for p in particles]
        particles = removeCollided(particles)
    print(f"number of particles after resolving collisions: {len(particles)}")




