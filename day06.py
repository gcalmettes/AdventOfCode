"""
http://adventofcode.com/2017/day/6
"""

import numpy as np
from collections import defaultdict

INPUT = """14	0	15	12	11	11	3	5	1	6	8	4	9	1	8	4"""

# part 1
def getInitialMemory(s):
    return [int(x) for x in s.split()]

def getNewMemoryAllocation(allocation):
    maxBlocks = np.max(allocation)
    idx = np.where(allocation == maxBlocks)[0][0]
    newAllocation = allocation
    newAllocation[idx] = 0
    while maxBlocks>0:
        idx = (idx + 1) % len(newAllocation)
        newAllocation[idx]+=1
        maxBlocks -= 1
    return tuple(newAllocation)

def reallocateMemory(s):
    allocation = getInitialMemory(s)
    allStates = defaultdict(int)
    allStates[tuple(allocation)] += 1
    notSeen = True

    while notSeen:
        newAllocation = getNewMemoryAllocation(allocation)
        allStates[newAllocation] += 1
        if allStates[newAllocation] > 1:
            notSeen=False

    return len(allStates)


# part2

def reallocateMemoryCycle(s):
    allocation = getInitialMemory(s)
    allStates = defaultdict(int)
    allStates[tuple(allocation)] = np.array([1, len(allStates)+1])
    notSeen = True

    while notSeen:
        newAllocation = getNewMemoryAllocation(allocation)
        allStates[newAllocation] += np.array([1, len(allStates)+1])
        if allStates[newAllocation][0] > 1:
            notSeen=False

    return (len(allStates) + 1) - (allStates[newAllocation][1]-(len(allStates)+1)) + 1



if __name__ == "__main__":
    # part 1
    print(reallocateMemory(INPUT))
    # part 2
    print(reallocateMemoryCycle(INPUT))
