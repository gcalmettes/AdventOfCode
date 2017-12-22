"""
http://adventofcode.com/2017/day/21
"""

import copy
import functools
from typing import Tuple, List

def convertInput(input: str) -> List[List[str]]:
    return [[c for c in line] for line in input.split("\n")]

def getRule(rule: str) -> Tuple[List[List[str]], List[List[str]]]:
    pattern, transform = rule.strip().split(" => ")
    pattern = [[c for c in line] for line in pattern.split("/")]
    transform = [[c for c in line] for line in transform.split("/")]
    return (pattern,transform)

def getAllRotations(pattern: List[List[str]]) -> Tuple[List[List[str]]]:
    fh = copy.deepcopy(pattern) # horizontal flip
    fv = [[] for line in pattern] # vertical flip
    r180 = [[] for line in pattern] # horizontal + vertical flip (rotation 180)
    r90 = copy.deepcopy(pattern) # rotation 90
    r90h = copy.deepcopy(pattern) # rotation 270
    for i,line in enumerate(pattern):
        fh[i].reverse()
        fv[i] = pattern[len(pattern)-1-i]
        r180[i] = fh[len(pattern)-1-i]
        r90[i] = [pattern[len(pattern)-1-j][i] for j in range(len(pattern))]
        r90h[i] = [pattern[j][i] for j in range(len(pattern))]
    r270h = [r90[len(pattern) - 1 - i] for i in range(len(pattern))]
    r270 = [r90h[len(pattern) - 1 - i] for i in range(len(pattern))]
    return (pattern, fh, fv, r90, r90h, r180, r270, r270h)

def divideSquare(pattern: List[List[str]], n: int) -> List[List[str]]:
    size = len(pattern)
    if size>1:
        assert len(pattern)%n == 0
    if n == 2:
        # two-by-two pixels
        dividedPattern = [ [ pattern[i*n][j*n:n+j*n], pattern[i*n+1][j*n:n+j*n] ] for i in range(size//n) for j in range(size//n)]
    elif n == 3:
        # three-by-three pixels
        dividedPattern = [ [ pattern[i*n][j*n:n+j*n], pattern[i*n+1][j*n:n+j*n], pattern[i*n+2][j*n:n+j*n] ] for i in range(size//n) for j in range(size//n)]

    return dividedPattern

def concatenatePixels(pixels: List[List[str]], nCols:int) -> List[List[str]]:
    pixelSize = len(pixels[0])
    totalSize = len(pixels)*pixelSize

    print(f"pixelSize: {pixelSize}, totalSize: {totalSize}, cols: {nCols}, lines: {totalSize//nCols} ")    

    lines = [ [] for _ in range(totalSize//nCols)]

    for i in range(totalSize//nCols):
        lines[i] = functools.reduce((lambda a, b: a + b), [pixels[k + (i//pixelSize) * nCols][i%pixelSize] for k in range(nCols)])

    return lines

def fractilize(startingPattern: List[List[str]], rules: List[List[str]]) -> List[List[str]]:
    pattern = copy.deepcopy(startingPattern)
    size = len(pattern)
    print(f"### fractilize size entry: {size}")
    print("-- Division --")
    if size%2 == 0:
        pixelSize = 2
        divided = divideSquare(pattern, pixelSize)
        print("will divide in 2 by 2")
    elif size%3 == 0:
        pixelSize = 3
        divided = divideSquare(pattern, pixelSize)
        print("will divide in 3 by 3")
    else:
        print(f"wrong size: {size}")
        return [[]]
    nPixels = len(divided)
    print(f"obtained {nPixels} pixels of size {len(divided[0][0])}x{len(divided[0][0])}")
    output = [[] for i in range(nPixels)]
    for j in range(nPixels):
        toApply = [rule for k,rule in enumerate(rules) if divided[j] in getAllRotations(rules[k][0])]
        if len(toApply)<=0:
            print(f"no match for pattern {j}")
            print(divided[j])
            return pattern, divided
        output[j]= toApply[0][1]
    print(f"matched and transformed {len(output)} pixels to size {len(output[0][0])}x{len(output[0][0])}")
    print("-- New pattern concatenation --")
    pattern = concatenatePixels(output, len(pattern[0])//pixelSize)
    print(f"### fractilize size exit: {len(pattern)}")
    return pattern

def iterate(startingPattern: List[List[str]], rules: List[List[str]], n: int = 5):
    i = 0
    pattern = copy.deepcopy(startingPattern)
    while i<n:
        print(" ")
        print(f"iteration {i}")
        pattern = fractilize(pattern, rules)
        i+=1
    return pattern

def countPixels(pattern: List[List[str]]) -> int:
    count = 0
    for i in range(len(pattern)):
        count += sum([c == "#" for c in pattern[i]])
    return count

START = """.#.
..#
###"""

TEST_RULES = """../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#"""
allRules = [getRule(pattern) for pattern in TEST_RULES.split("\n")]
assert countPixels(iterate(convertInput(START), allRules, 2)) == 12


if __name__ == "__main__":

    with open("day21_input.txt") as f:
        patterns = f.readlines()
    allRules = [getRule(pattern) for pattern in patterns]

    # part 1
    input = convertInput(START)
    final_part1 = iterate(input, allRules, 5)
    print(countPixels(final_part1))

    final_part2 = iterate(input, allRules, 18)
    print(countPixels(final_part2 ))


    