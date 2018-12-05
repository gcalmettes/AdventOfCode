"""
http://adventofcode.com/2017/day/22
"""

from typing import List, Tuple, NamedTuple

class virusCarrier(NamedTuple):
	i: int
	j: int
	grid: List[List[str]]
	direction: str
	infections: int = 0

def parseGrid(input: List[str]) -> List[List[str]]:
	return [list(line) for line in input]

def getStartingPosition(grid: List[List[str]]) -> Tuple[int, int]:
	nRows = len(grid)
	nCols = len(grid[0])
	
	return (nRows-1)//2, (nCols-1)//2

def changeNodeState(node: str) -> str:
	if node == ".":
		newState = "#"
	elif node == "#":
		newState = "."
	else:
		raise ValueError(f"The node has an unknown state: {node}")
	return newState

def getDirection(currentDir: str, turn: str) -> Tuple[str, Tuple[int, int]]:
	directions = ["up", "right", "down", "left"]
	coords = [(-1, 0), (0, 1), (1, 0), (0, -1)]
	idx = directions.index(currentDir)
	if turn == "right":
		dirIdx = (idx+1)%4
	elif turn == "left":
		dirIdx = (idx-1)%4
	return directions[dirIdx], coords[dirIdx]

def extendGrid(grid: List[List[str]], n: int) -> List[List[str]]:
	nRows = len(grid)
	nCols = len(grid[0])

	newGrid = [["."]*n for _ in range(nRows + 2*n)]
	for i in range(nRows+2*n):
		if i<n or i>=nRows+n:
			newGrid[i] = newGrid[i] + ["."]*(nCols+n)
		else:
			newGrid[i] = newGrid[i] + grid[i-n] + ["."]*n
	return newGrid

def step(virus: virusCarrier) -> virusCarrier:
	grid = virus.grid
	i,j = virus.i, virus.j
	
	# if virus is on the border, extend grid by 5 
	if i < 2 or i > len(grid)-3 or j < 2 or j > len(grid[0])-3:
		newSize = (len(grid) + 2 * 5, len(grid[0])+ 2 * 5)
		grid = extendGrid(grid, 5)
		assert (len(grid), len(grid[0])) == newSize
		i = i + 5
		j = j + 5
	
	node = grid[i][j]
	
	if node == ".": # clean, infect node and turn left
		grid[i][j] = changeNodeState(grid[i][j])
		newDir, deltaMove = getDirection(virus.direction, "left")
		return virusCarrier(i+deltaMove[0], j+deltaMove[1], grid, newDir, virus.infections+1)
	elif node == "#": # infected, clean node and turn right
		grid[i][j] = changeNodeState(grid[i][j])
		newDir, deltaMove = getDirection(virus.direction, "right")
		return virusCarrier(i+deltaMove[0], j+deltaMove[1], grid, newDir, virus.infections)


def runSimulation(grid: List[List[str]], n: int) -> int:
	start = getStartingPosition(grid)
	virus = virusCarrier(start[0], start[1], grid, "up")
	for itertion in range(n):
		virus = step(virus)
	return virus.infections


################
## for part 2

def changeNodeState2(node: str) -> str:
	if node == ".":
		newState = "W"
	elif node == "W":
		newState = "#"
	elif node == "#":
		newState = "F"
	elif node == "F":
		newState = "."
	else:
		raise ValueError(f"The node has an unknown state: {node}")
	return newState

def getDirection2(currentDir: str, turn: str) -> Tuple[str, Tuple[int, int]]:
	directions = ["up", "right", "down", "left"]
	coords = [(-1, 0), (0, 1), (1, 0), (0, -1)]
	idx = directions.index(currentDir)
	if turn == "right":
		dirIdx = (idx+1)%4
	elif turn == "left":
		dirIdx = (idx-1)%4
	elif turn == "none":
		dirIdx = idx
	elif turn == "reverse":
		dirIdx = (idx+2)%4
	return directions[dirIdx], coords[dirIdx]


def step2(virus: virusCarrier) -> virusCarrier:
	grid = virus.grid
	i,j = virus.i, virus.j
	
	# if virus is on the border, extend grid by 5 
	if i < 2 or i > len(grid)-3 or j < 2 or j > len(grid[0])-3:
		newSize = (len(grid) + 2 * 5, len(grid[0])+ 2 * 5)
		grid = extendGrid(grid, 5)
		assert (len(grid), len(grid[0])) == newSize
		i = i + 5
		j = j + 5
	
	node = grid[i][j]
	
	if node == ".": # clean, weaken node and turn left
		grid[i][j] = changeNodeState2(grid[i][j])
		newDir, deltaMove = getDirection2(virus.direction, "left")
		return virusCarrier(i+deltaMove[0], j+deltaMove[1], grid, newDir, virus.infections)
	elif node == "W": # weakened, infect node and do not turn
		grid[i][j] = changeNodeState2(grid[i][j])
		newDir, deltaMove = getDirection2(virus.direction, "none")
		return virusCarrier(i+deltaMove[0], j+deltaMove[1], grid, newDir, virus.infections+1)
	elif node == "#": # infected, flag node and turn right
		grid[i][j] = changeNodeState2(grid[i][j])
		newDir, deltaMove = getDirection2(virus.direction, "right")
		return virusCarrier(i+deltaMove[0], j+deltaMove[1], grid, newDir, virus.infections)
	elif node == "F":
		grid[i][j] = changeNodeState2(grid[i][j])
		newDir, deltaMove = getDirection2(virus.direction, "reverse")
		return virusCarrier(i+deltaMove[0], j+deltaMove[1], grid, newDir, virus.infections)

def runSimulation2(grid: List[List[str]], n: int) -> int:
	start = getStartingPosition(grid)
	virus = virusCarrier(start[0], start[1], grid, "up")
	for iteration in range(n):
		virus = step2(virus)
	return virus.infections


TEST_GRID = """..#
#..
..."""

testgrid = parseGrid(TEST_GRID.split("\n"))
assert runSimulation(testgrid, 70) == 41
testgrid = parseGrid(TEST_GRID.split("\n"))
assert runSimulation(testgrid, 10000) == 5587


testgrid = parseGrid(TEST_GRID.split("\n"))
assert runSimulation2(testgrid, 100) == 26

if __name__ == "__main__":
	with open("day22_input.txt", "r") as f:
		lines = f.read().split("\n")
	
	# part 1
	grid = parseGrid(lines)
	print(runSimulation(grid, 10000))

	# part 2
	grid = parseGrid(lines)
	print(runSimulation2(grid, 10000000))

	


	