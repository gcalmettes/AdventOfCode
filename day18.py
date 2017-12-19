"""
http://adventofcode.com/2017/day/18
"""

from typing import List, Dict, NamedTuple

class Register:

    def __init__(self, instructions: List[str]) -> None:
        self.instructions: List[List[str]] = [self.readInstruction(ins) 
                                        for ins in instructions]
        self.pos: int = 0
        self.registers: Dict[str, int] = self.initializeRegisters(self.instructions)
        self.frequency: int = None
        self.run = True
        # print(self.instructions)

    def readInstruction(self, line: str) -> List[str]:
        return line.split()

    def initializeRegisters(self, input: List[List[str]]) -> Dict[str, int]:
        setCalls = set()
        for instruction in input:
            if instruction[0] == "set":
                setCalls.add(instruction[1])
        return {register: 0 for register in setCalls}
    
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
        elif instruction[0] == "add":
            try:
                valToAdd = int(instruction[2])
            except:
                valToAdd =  self.registers[instruction[2]]
            self.registers[instruction[1]] +=  valToAdd
            return 1
        elif instruction[0] == "mul":
            try:
                factor = int(instruction[2])
            except:
                factor =  self.registers[instruction[2]]
            self.registers[instruction[1]] *=  factor
            return 1
        elif instruction[0] == "mod":
            try:
                newVal = self.registers[instruction[1]] % int(instruction[2])
            except:
                newVal =  self.registers[instruction[1]] % self.registers[instruction[2]]
            self.registers[instruction[1]] = newVal
            return 1
        elif instruction[0] == "jgz":
            if self.registers[instruction[1]] != 0:
                try:
                    idx = int(instruction[2])
                except:
                    idx = self.registers[instruction[2]]
                return idx
            else:
                return 1
        # sound related action
        elif instruction[0] == "snd":
            # update frequency
            self.frequency = self.registers[instruction[1]]
            # do next instruction
            return 1
        elif instruction[0] == "rcv":
            if self.registers[instruction[1]] != 0:
                # print(self.frequency)
                self.run = False
                print(f"Recovered frequency: {self.frequency}")
            else: # if zero do next instruction
                return 1

    def playAll(self):
        while self.run:
            nextMove = self.play(self.pos)
            if self.run == False:
                return self.frequency
                break
            self.pos = self.pos + nextMove


# TEST_INPUT = """set a 1
# add a 2
# mul a a
# mod a 5
# snd a
# set a 0
# rcv a
# jgz a -1
# set a 1
# jgz a -2"""

# TEST_INSTRUCTION = TEST_INPUT.split("\n")
# TEST_REGISTERS = Register(TEST_INSTRUCTION)

# assert TEST_REGISTERS.registers == {"a": 0}
# assert TEST_REGISTERS.playAll() == 4


class RegisterCom:

    def __init__(self, instructions: List[str], p: int) -> None:
        self.instructions: List[List[str]] = [self.readInstruction(ins) 
                                        for ins in instructions]
        self.pos: int = 0
        self.registers: Dict[str, int] = self.initializeRegisters(self.instructions)
        self.registers["p"] = p
        self.queue: List[int] = []
        self.inPause = False
        self.comWith = None
        self.nSend = 0

    def readInstruction(self, line: str) -> List[str]:
        return line.split()

    def initializeRegisters(self, input: List[List[str]]) -> Dict[str, int]:
        setCalls = set()
        for instruction in input:
            if instruction[0] == "set":
                setCalls.add(instruction[1])
        return {register: 0 for register in setCalls}
    
    def addCommunicationWith(self, register) -> None:
        self.comWith = register

    def play(self, pos: int) -> int:
        """
        Do instruction.
        If not an act, then do instruction[self.pos+1]
        """
        
        if len(self.queue) > 0:
            self.inPause = False

        instruction = self.instructions[pos]

        # direct action on register
        if instruction[0] == "set":
            try:
                newVal = int(instruction[2])
            except:
                newVal =  self.registers[instruction[2]]
            self.registers[instruction[1]] =  newVal
            return 1
        elif instruction[0] == "add":
            try:
                valToAdd = int(instruction[2])
            except:
                valToAdd =  self.registers[instruction[2]]
            self.registers[instruction[1]] +=  valToAdd
            return 1
        elif instruction[0] == "mul":
            try:
                factor = int(instruction[2])
            except:
                factor =  self.registers[instruction[2]]
            self.registers[instruction[1]] *=  factor
            return 1
        elif instruction[0] == "mod":
            try:
                newVal = self.registers[instruction[1]] % int(instruction[2])
            except:
                newVal =  self.registers[instruction[1]] % self.registers[instruction[2]]
            self.registers[instruction[1]] = newVal
            return 1
        elif instruction[0] == "jgz":
            try:
                val = self.registers[instruction[1]]
            except:
                val = int(instruction[1])
            if val > 0:
                try:
                    idx = int(instruction[2])
                except:
                    idx = self.registers[instruction[2]]
                return idx
            else:
                return 1

        # communication related action
        elif instruction[0] == "snd":
            try:
                valToSend = int(instruction[1])
            except:
                valToSend = self.registers[instruction[1]]
            # update queue of other register
            self.comWith.queue.append(valToSend)
            self.nSend += 1
            # do next instruction
            return 1
        elif instruction[0] == "rcv":
            try:
                valToReceive = self.queue.pop(0)
                self.registers[instruction[1]] = valToReceive
                return 1
            except:
                # waiting for value to be added to queue
                self.inPause = True
                # print("register in Pause")
                return 0
            

def playParrallel(register0, register1):
    bothInPause = False

    while not bothInPause:
        
        try:
            reg0nextMove = register0.play(register0.pos)
        except:
            print(f"Reg0 error at pos {register0.pos}: {register0.instructions[register0.pos]}")
            bothInPause = True

        try:
            reg1nextMove = register1.play(register1.pos)
        except:
            print(f"Reg1 error at pos {register1.pos}: {register1.instructions[register0.pos]}")
            bothInPause = True

        if register0.inPause == True and register1.inPause == True:
            bothInPause = True
            break

        register0.pos = register0.pos + reg0nextMove
        register1.pos = register1.pos + reg1nextMove
        


if __name__ == "__main__":
    with open("day18_input.txt") as f:
        instructions = f.readlines()
    
    # part 1
    registers = Register(instructions)
    registers.playAll() 

    # part 2
    register0 = RegisterCom(instructions, p=0)
    register1 = RegisterCom(instructions, p=1)
    register0.addCommunicationWith(register1)
    register1.addCommunicationWith(register0)
    playParrallel(register0, register1)
    print(f"n sent by program1: {register1.nSend}")

