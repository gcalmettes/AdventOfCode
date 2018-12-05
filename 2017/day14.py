"""
http://adventofcode.com/2017/day/14
"""

from day10 import makeHash
from typing import List


def getHashInputs(key: str, nRows: int = 128) -> List[str]:
    return [f"{key}-{i}" for i in range(nRows)]

def binarize(hex: str) -> str:
    return "".join(f"{int(c,16):04b}" for c in hex)

def convertHashToBits(key: str) -> List[str]:
    hashInputs = getHashInputs(key)
    knotHashed = [makeHash(s) for s in hashInputs]
    assert [len(input) for input in knotHashed] == [32] * len(knotHashed)
    knotBits = [binarize(input) for input in knotHashed]
    assert [len(input) for input in knotBits] == [128] * len(knotBits)
    return knotBits

def countOnes(key: str):
    knotBits = convertHashToBits(key)
    ones = [len([bit for bit in knot if bit == "1"]) for knot in knotBits]
    return sum(ones)

def makeGrid(key: str) -> List[List[str]]:
    bits = convertHashToBits(key)
    grid =[]
    for knot in bits:
        grid.append([c for c in knot])
    return grid


def clearRegion(grid: List[List[str]], i: int, j: int) -> None:
    nRows = len(grid)
    nCols = len(grid[0])

    if i < 0 or i >= nRows or j < 0 or j >= nCols:
        # out of the grid
        return
    elif grid[i][j] == "0":
        # not part of a region
        return
    else:
        # part of a region, set to "0" and check neighbors
        grid[i][j] = "0"
        clearRegion(grid, i+1, j)
        clearRegion(grid, i-1, j)
        clearRegion(grid, i, j+1)
        clearRegion(grid, i, j-1)


def countRegions(key: str) -> int:
    grid = makeGrid(key)[:]
    
    nRegions = 0

    for i,row in enumerate(grid):
        for j,col in enumerate(row):
            if grid[i][j] == "1":
                clearRegion(grid, i, j)
                nRegions += 1
    return nRegions



if __name__ == '__main__':

    # part 1
    INPUT = """ugkiagan"""
    print(countOnes(INPUT))

    # part 2
    print(countRegions(INPUT))
