"""
http://adventofcode.com/2017/day/17
"""

from typing import List


def getNextPosition(currentPosition: int, currentList: List[int], stepLength: int) -> int:
   listLenght = len(currentList)
   nextPosition = (currentPosition + stepLength) % listLenght
   return nextPosition


def insertAtPosition(toInsert: int, currentList: List[int], position: int) -> List[int]:
    newList = currentList[:]
    newList.insert(position+1, toInsert)
    return newList

def spinlock(stepLength: int, nStep: int = 2017) -> int:
    buffer = [0]
    currPos = 0

    for i in range(1, nStep+1):
        currPos = getNextPosition(currPos, buffer, stepLength)
        buffer = insertAtPosition(i, buffer, currPos)
        currPos += 1
    
    return buffer[(currPos+1)%len(buffer)]

def getValAfterZero(stepLength: int, nStep: int = 50000000) -> int:
    """
    The idea here is to just care about the value if it is inserted
    after the position 0 which holds Zero, and not really constructing
    the list itself
    """
    currentPosition = 0
    listLength = 1

    valAfterZero = 0

    for i in range(1, nStep + 1):
        currentPosition = (currentPosition + stepLength) % listLength
        listLength += 1 # like if we inserted a number
        currentPosition +=1 # the position of the new inserted number
        if currentPosition == 1:
            valAfterZero = i
    
    return valAfterZero


TEST_LENGTH = 3
assert spinlock(TEST_LENGTH) == 638

if __name__ == '__main__':
    
    INPUT = 354

    # part 1
    print(spinlock(INPUT, 2017))

    # part 2
    print(getValAfterZero(INPUT, 50000000))