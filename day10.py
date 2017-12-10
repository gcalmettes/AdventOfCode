"""
http://adventofcode.com/2017/day/10
"""

import numpy as np

def makeKnot(numberList, position, length):
	listToKnot = np.array(numberList[:])
	indices = np.arange(position, position+length)%len(numberList)
	toReverse = listToKnot[indices]
	reversedList = list(reversed(toReverse))
	listToKnot[indices] = reversedList
	return listToKnot

assert list(makeKnot([0, 1, 2, 3, 4], 0, 3)) == [2, 1, 0, 3, 4]
assert list(makeKnot([2, 1, 0, 3, 4], 3, 4)) == [4, 3, 0, 1, 2]

def makeAllKnots(numberList, lengthList):
	skipSize = 0
	currentPosition = 0
	for length in lengthList:
		numberList = makeKnot(numberList, currentPosition, length)
		currentPosition = (currentPosition + skipSize + length)%len(numberList)
		skipSize += 1
	return numberList

assert list(makeAllKnots([0, 1, 2, 3, 4], [3, 4, 1, 5])) == [3, 4, 2, 1, 0]

def convert(sparseHash):
	denseHash = [np.bitwise_xor.reduce(sparseHash[i*16:(i+1)*16]) for i in range(16)]
	hashKnot = ""
	for knot in denseHash:
		hexKnot = hex(knot)[2:] #remove 0x
		if len(hexKnot)<2:
			hexKnot = "0"+hexKnot
		hashKnot = hashKnot+hexKnot
	return hashKnot



if __name__ == "__main__":
	INPUT = """189,1,111,246,254,2,0,120,215,93,255,50,84,15,94,62"""

	# part 1
	lengths = [int(length) for length in INPUT.split(",")]
	finalHash = makeAllKnots(np.arange(256), lengths)
	# print(finalKnot)
	print(finalHash[0]*finalHash[1])

	# part 2
	EXTRA_LENGTHS = """17,31,73,47,23"""
	lengths2 = [int(ord(character)) for character in INPUT] + [int(length) for length in EXTRA_LENGTHS.split(",")]
	lengths2 = lengths2 * 64
	sparseHash = makeAllKnots(np.arange(256), lengths2)
	print(convert(sparseHash))
