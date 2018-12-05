"""
http://adventofcode.com/2017/day/12
"""

from typing import Tuple, List, Dict

def getConections(program: str) -> Tuple:
	programID,programConnections = program.split("<->")
	id = int(programID.strip())
	connections = [int(c.strip()) for c in programConnections.split(",")]
	return id,connections

def mapConnections(programList: List[str]) -> Dict:
	programMap = {}
	for program in programList:
		programID,programConnections = getConections(program)
		programMap[programID] = programConnections
	return programMap


def crawlNetwork(programList: List[str], program: int) -> Dict:
	programMap = mapConnections(programList)
	
	toCheck = [program]

	depth = 0
	connectogram: Dict = {}
	moreConnections = True

	while moreConnections:
		directConnections = [item for prog in toCheck for item in programMap[prog] if item not in [val for values in connectogram.values() for val in values]]
		if len(directConnections)>0:
			connectogram[depth] = directConnections
			depth += 1
			toCheck = directConnections
		else:
			moreConnections = False

	return connectogram


testData = """0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"""

assert len(set([val for values in crawlNetwork([line.strip() for line in testData.split("\n")], 0).values() for val in values])) == 6


def getAllGroups(programList: List[str], startWith:int = 0) -> int:
	programMap = mapConnections(programList)

	nbGroups:int = 1 # at least the group of the program startWith

	startingGroup = set([prg for programs in crawlNetwork(programList, startWith).values() for prg in programs])
	leftPrograms = [prg for prg in programMap if prg not in startingGroup]
	checkedPrograms = [prg for prg in startingGroup]

	while len(leftPrograms)>0:
		nbGroups += 1
		startWith = leftPrograms[0]
		startingGroup = set([prg for programs in crawlNetwork(programList, startWith).values() for prg in programs])
		checkedPrograms = checkedPrograms + [prg for prg in startingGroup]
		leftPrograms = [prg for prg in programMap if prg not in checkedPrograms]
		
	return nbGroups



if __name__ == '__main__':
	with open("day12_input.txt") as f:
		lines = [line.strip() for line in f.readlines()]
	# lines = [line.strip() for line in testData.split("\n")]
	
	# part 1
	connectogram = crawlNetwork(lines, 0)
	print(len(set([val for values in connectogram.values() for val in values])))

	# part 2
	print(getAllGroups(lines))

		
	


