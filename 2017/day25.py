"""
http://adventofcode.com/2017/day/25
"""

import re
from typing import NamedTuple, Tuple, List, Dict

class State(NamedTuple):
    name: str
    moveZero: Tuple[int] # newValue, deltaPos, nextState
    moveOne: Tuple[int]

class Turing:
    def __init__(self, startingState, checkAt, stateDict):
        self.tape = [0]
        self.pos = 0
        self.nStep = 0
        self.stateDict: Dict[str, State] = stateDict
        self.state: str = startingState
        self.checkAt: int = checkAt

    def step(self):
        if self.tape[self.pos] == 0:
            newVal,deltaPos,nexState = self.stateDict[self.state].moveZero
            nextPos = self.pos + deltaPos
            self.tape[self.pos] = newVal
            self.pos = self._checkPos(nextPos)
            self.state = nexState
        elif self.tape[self.pos] == 1:
            newVal,deltaPos,nexState = self.stateDict[self.state].moveOne
            nextPos = self.pos + deltaPos
            self.tape[self.pos] = newVal
            self.pos = self._checkPos(nextPos)
            self.state = nexState
        self.nStep += 1

    def _checkPos(self, nextPos):
        if nextPos == len(self.tape):
            self.tape.append(0)
            return self.pos + 1
        elif nextPos == -1:
            self.tape.insert(0, 0)
            return 0
        else:
            return nextPos

def convertDirection(dir: str) -> int:
    if dir == "right":
        return 1
    elif dir == "left":
        return -1

def checkSum(turing: Turing) -> int:
    return sum(turing.tape)

            
def parseInstructions(input: str) -> Dict[str, State]:
    regex1 = r"Begin in state ([A-Z]).\nPerform a diagnostic checksum after (\d+) steps."
    regex2 = r"In state ([A-Z]):\n\s+If the current value is 0:\n\s+- Write the value (0|1).\n\s+- Move one slot to the ([a-z]+).\n\s+- Continue with state ([A-Z]).\n\s+If the current value is 1:\n\s+- Write the value (0|1).\n\s+- Move one slot to the ([a-z]+).\n\s+- Continue with state ([A-Z])."
    startingState,checkAt = re.findall(regex1, input)[0]
    stateDict = {state: State(state, (int(newValue0), convertDirection(deltaPos0), nextState0), (int(newValue1), convertDirection(deltaPos1), nextState1)) \
                    for [state, newValue0, deltaPos0, nextState0, newValue1, deltaPos1, nextState1] in re.findall(regex2, input)}

    return Turing(startingState, int(checkAt), stateDict)



if __name__ == "__main__":
    with open("day25_input.txt") as f:
        raw = f.read()
    # print(raw)

    turing = parseInstructions(raw)
    while turing.nStep < turing.checkAt:
        turing.step()
    print(checkSum(turing))