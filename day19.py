"""
http://adventofcode.com/2017/day/19
"""

import string
from typing import List, NamedTuple, Tuple


class Path:
    def __init__(self, row: int, col: int):
        self.row = row
        self.col = col
        self.dir = (1, 0)
        self.letters = []
        self.steps = 1

def getStart(inputMap: List[str]) -> Tuple[int, int]:
    col = list(i for i,sign in enumerate(inputMap[0]) if sign=="|")[0]
    return (0, col)

def navigateGrid(inputMap: List[str]):
    startingPt= getStart(inputMap)
    pathLeft = True

    path = Path(startingPt[0], startingPt[1])
    path.row += 1


    nRows = len(inputMap)
    nCols = len(inputMap[0])

    while pathLeft:
        nextPt = inputMap[path.row][path.col]
        # print(nextPt)
        
        if nextPt == "|" or nextPt == "-":
            # continue horizontal or vertical movement
            path.row += path.dir[0]
            path.col += path.dir[1]
            path.steps+=1
        
        elif nextPt == "+":
            # change direction
            if path.dir[0] != 0: # if it was vertical movement
                try:
                    nextStepRight = inputMap[path.row][path.col+1]
                except IndexError:
                    nextStepRight = " "
                try:
                    nextStepLeft = inputMap[path.row][path.col-1]
                except IndexError:
                    nextStepLeft = " "

                if nextStepRight != " " and nextStepLeft == " ":
                    path.dir = (0, 1) # go to the right
                    path.row += path.dir[0]
                    path.col += path.dir[1]
                    
                elif nextStepRight == " " and nextStepLeft != " ":
                    path.dir = (0, -1) # go to the left
                    path.row += path.dir[0]
                    path.col += path.dir[1]
                else:
                    print(f"Unknown direction")
                path.steps+=1

            elif path.dir[1] != 0: # if it was horizontal movement
                try:
                    nextStepDown = inputMap[path.row+1][path.col]
                except IndexError:
                    nextStepDown = " "
                try:
                    nextStepUp = inputMap[path.row-1][path.col]
                except IndexError:
                    nextStepUp = " "

                if nextStepDown != " " and nextStepUp == " ":
                    path.dir = (1, 0) # go down
                    path.row += path.dir[0]
                    path.col += path.dir[1]
                elif nextStepDown == " " and nextStepUp != " ":
                    path.dir = (-1, 0) # go up
                    path.row += path.dir[0]
                    path.col += path.dir[1]
                else:
                    print(f"Unknown direction")
                path.steps+=1

        elif nextPt in string.ascii_letters:
            print("... adding letter", nextPt)
            path.letters.append(nextPt)
            path.row += path.dir[0]
            path.col += path.dir[1]
            path.steps+=1
        else:
            print("Navigation terminated")
            pathLeft = False

        if (path.row >= len(inputMap) or path.row < 0
              or path.col >= len(inputMap[0]) or path.col < 0):
            print("out of the grid")
            pathLeft = False
    return path


TEST_INPUT = """     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
"""
assert navigateGrid(TEST_INPUT.split("\n")).letters == ["A", "B", "C", "D", "E", "F"]


if __name__ == "__main__":
    with open("day19_input.txt") as f:
        inputMap = f.readlines()

    path = navigateGrid(inputMap)

    # part 1
    print("letters collected:", "".join(path.letters))

    # part 2
    print("numbers of steps:", path.steps)
    