"""
http://adventofcode.com/2017/day/11
"""

from typing import NamedTuple

class Position(NamedTuple):
	x: int = 0
	y: int = 0
	distanceAway: int = 0
	maxDistanceAway: int = 0

def moveStep(currentPosition: Position, step: str) -> Position:
	if step == "n":
		x,y = 0,2
	elif step == "ne":
		x,y = 1,1
	elif step == "se":
		x,y = 1,-1
	elif step == "s":
		x,y = 0,-2
	elif step == "sw":
		x,y = -1,-1
	elif step == "nw":
		x,y = -1,1
	distanceAway = calculDistanceAway(currentPosition.x+x, currentPosition.y+y)
	maxDistanceAway = distanceAway if distanceAway > currentPosition.maxDistanceAway else currentPosition.maxDistanceAway
	return Position(currentPosition.x+x, currentPosition.y+y, distanceAway, maxDistanceAway)

def movePath (currentPosition: Position, path: str) -> Position:
	steps = path.split(",")
	pos = currentPosition
	for step in steps:
		pos = moveStep(pos, step)
	return pos

def calculDistanceAway(x: int, y: int) -> int:
	if abs(y) > abs(x):
		distance =  (abs(x) + abs(y))//2
	else:
		horizontalStep = abs(x) - abs(y)
		distance = abs(y) + horizontalStep
	return distance


testPosition = Position()
assert movePath(testPosition, "ne,ne,ne").distanceAway == 3
assert movePath(testPosition, "se,se,se").distanceAway == 3
assert movePath(testPosition, "ne,ne,sw,sw").distanceAway == 0
assert movePath(testPosition, "ne,ne,s,s").distanceAway == 2
assert movePath(testPosition, "se,sw,se,sw,sw").distanceAway == 3
assert movePath(testPosition, "s,s,s,s,n,s").distanceAway == 4
assert movePath(testPosition, "se,se,se,se,se,s,nw").distanceAway == 5
assert movePath(testPosition, "ne,se,s,nw,n,ne,nw,sw").distanceAway == 1
assert movePath(testPosition, "se,se,se,se,se,ne,ne,ne,ne,ne").distanceAway == 10



if __name__ == '__main__':
	with open("day11_input.txt") as f:
		path = f.read().strip()
	pos = Position()
	pos = movePath(pos, path)
	# part 1
	print(pos.distanceAway)
	# part 2
	print(pos.maxDistanceAway)
	


