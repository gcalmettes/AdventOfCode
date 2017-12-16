"""
http://adventofcode.com/2017/day/16
"""

import re
from typing import List


def danceMove(programs: str, move: str) -> str:
    regex = r"([s|x|p])([0-9a-z]+)/?([0-9a-z]+)?"
    moveType,pos1,pos2 = re.match(regex, move).groups()
    if moveType == "s":
        output = programs[-int(pos1):]+programs[:-int(pos1)]
    elif moveType == "x":
        pos1,pos2 = sorted([int(pos1), int(pos2)])
        output = programs[:pos1] + programs[pos2] + programs[pos1+1:pos2] + programs[pos1] + programs[pos2+1:]
    elif moveType == "p":
        pos1,pos2 = sorted([i for i,letter in enumerate(programs) if letter in [pos1, pos2]])
        output = programs[:pos1] + programs[pos2] + programs[pos1+1:pos2] + programs[pos1] + programs[pos2+1:]
    return output

def danceMove2(programs: str, move: str) -> str:
    moveType,action = move[0],move[1:]
    if moveType == "s":
        output = programs[-int(action):]+programs[:-int(action)]
    elif moveType == "x":
        pos1,pos2 = sorted([int(i) for i in action.split("/")])
        output = programs[:pos1] + programs[pos2] + programs[pos1+1:pos2] + programs[pos1] + programs[pos2+1:]
    elif moveType == "p":
        pos1,pos2 = sorted([i for i,letter in enumerate(programs) if letter in action.split("/")])
        output = programs[:pos1] + programs[pos2] + programs[pos1+1:pos2] + programs[pos1] + programs[pos2+1:]
    return output

def makeDance(programs: str, moveList: List[str]) -> str:
    programs = programs[:]
    for move in moveList:
        programs = danceMove(programs, move)
    return programs

TEST_INPUT ="abcde"
TEST_MOVES = ["s1", "x3/4", "pe/b"]

assert makeDance(TEST_INPUT, TEST_MOVES) == "baedc"


if __name__ == '__main__':
    
    programs_initial = "abcdefghijklmnop"

    with open("day16_input.txt") as f:
        moves = f.read().strip().split(",")
        
        # part 1
        programs = programs_initial[:]
        programs = makeDance(programs, moves)
        print(programs)
        
        # part 2

        # # computing was too long, so checked if input was cycling
        # # it was! every 42 iterations we ended up with the initial input
        # programs = programs_initial[:]
        # for i in range(1000000000):
        #     if programs==programs_initial:
        #         print(i)
        #     programs = makeDance(programs, moves)
        # # return 0, 42, 84, etc

        programs = programs_initial[:]
        for i in range(1000000000%42):
            programs = makeDance(programs, moves)
        
        print(programs)


        
