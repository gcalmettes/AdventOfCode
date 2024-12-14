import re
from typing import List
from dataclasses import dataclass
from PIL import Image

with open("inputs/14.in")  as f:
    input = f.read().strip().split("\n")

WIDTH = 101
HEIGHT = 103

@dataclass
class Robot:
    x: int
    y: int
    vx: int
    vy: int

    def step(self):
        self.x = abs((self.x+self.vx) % WIDTH)
        self.y = abs((self.y+self.vy) % HEIGHT)

def save_img(robots: List[Robot], step):
    img = Image.new(mode='L', size=(WIDTH, HEIGHT), color=255)
    rb = {(r.x, r.y) for r in robots}
    for x in range(img.size[0]): # for every pixel:
        for y in range(img.size[1]):
            if (x,y) in rb:
                # change to black
                img.putpixel((x, y), 0)
    img.save(f"{step}.png")

robots = []
for line in input:
    pattern = r'(-?\d+)'
    matches = re.findall(pattern, line)
    robots.append(Robot(*[int(m) for m in matches]))

# after visual inspection of all the images
p2 = 7916

for i in range(1, HEIGHT*WIDTH):
    for r in robots:
        r.step()
        if i == p2:
            save_img(robots, i)

midH = HEIGHT // 2
midW = WIDTH // 2

quadrants = {i: 0 for i in range(4)}

for r in robots:
    if r.x < midW and r.y < midH:
        quadrants[0] += 1
    if r.x > midW and r.y < midH:
        quadrants[1] += 1
    if r.x < midW and r.y > midH:
        quadrants[2] += 1
    if r.x > midW and r.y > midH:
        quadrants[3] += 1

p1 = 1
for _,n in quadrants.items():
    p1 *= n

print(f"part1: {p1}")
print(f"part2: {p2}")
