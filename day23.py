"""
http://adventofcode.com/2017/day/23
"""

from typing import List, Dict, NamedTuple
import numpy as np

class Register:

    def __init__(self, instructions: List[str]) -> None:
        self.instructions: List[List[str]] = [self.readInstruction(ins) 
                                        for ins in instructions]
        self.pos: int = 0
        self.registers: Dict[str, int] = {register: 0 for register in ["a", "b", "c", "d", "e", "f", "g", "h"]}
        self.count: int = 0
        self.run = True
        # print(self.instructions)

    def readInstruction(self, line: str) -> List[str]:
        return line.split()
    
    def play(self, pos: int) -> int:
        """
        Do instruction.
        If not an act, then do instruction[self.pos+1]
        """
        
        instruction = self.instructions[pos]

        # direct action on register
        if instruction[0] == "set":
            try:
                newVal = int(instruction[2])
            except:
                newVal =  self.registers[instruction[2]]
            self.registers[instruction[1]] =  newVal
            return 1
        elif instruction[0] == "sub":
            try:
                valToSub = int(instruction[2])
            except:
                valToSub =  self.registers[instruction[2]]
            self.registers[instruction[1]] -=  valToSub
            return 1
        elif instruction[0] == "mul":
            self.count += 1
            try:
                factor = int(instruction[2])
            except:
                factor =  self.registers[instruction[2]]
            self.registers[instruction[1]] *=  factor
            return 1
        elif instruction[0] == "jnz":
            try:
                offset = int(instruction[1])
            except:
                offset =  self.registers[instruction[1]]
            if offset != 0:
                try:
                    idx = int(instruction[2])
                except:
                    idx = self.registers[instruction[2]]
                return idx
            else:
                return 1

    def playAll(self):
        maxIdx = len(self.instructions)-1
        i = 0
        while self.run:
            nextMove = self.play(self.pos)
            if self.run == False:
                break
            if (self.pos + nextMove > maxIdx) or (self.pos + nextMove < 0):
                self.run = False
            else:
                self.pos = self.pos + nextMove
            i += 1
        return self.count


def isPrime(num):
    # Returns True if num is a prime number, otherwise False.
    # all numbers less than 2 are not prime
    if num < 2:
        return False

    # see if num is divisible by any number up to the square root of num
    for i in range(2, int(np.sqrt(num)) + 1):
        if num % i == 0:
            return False
    return True

def part2() -> int:
    a = 1
    b = 57 * 100 + 100000  # set b 57
    c = b + 17000  # set c b
    d = f = h = 0

    while 1:
        f = 1  # set f 1
        if not isPrime(b):
            h += 1 # sub h -1
        if b == c:  # jnz g 2
            break  # jnz 1 3
        b += 17  # sub b -17
    return h


if __name__ == "__main__":
    with open("day23_input.txt") as f:
        instructions = f.readlines()
    
    # part 1
    registers = Register(instructions)
    print(registers.playAll())
    
    # part 2
    print(part2())
    

