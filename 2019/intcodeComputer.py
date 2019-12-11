from typing import List, Tuple, Dict
from enum import Enum
from collections import defaultdict

class Opcode(Enum):
    ADD: int = 1
    MULTIPLY: int = 2
    INPUT: int = 3
    OUTPUT: int = 4
    JUMP_IF_TRUE: int = 5 
    JUMP_IF_FALSE: int = 6
    LESS_THAN: int = 7
    EQUALS: int = 8
    RELATIVE_BASE_OFFSET: int = 9
    HALT: int = 99

class ParameterMode:
    POSITION: int = 0
    IMMEDIATE: int = 1
    RELATIVE: int = 2

Program: List[int]
Modes = List[int]
Memory = defaultdict(int)


class IntcodeComputer:
    def __init__(self, program: List[int], inputs: List[int]=[]):
        self.memory: Memory = defaultdict(int, enumerate(program))
        self.inputs = inputs
        self.address = 0
        self.relative_base = 0

    @staticmethod
    def parse_opcode(opcode: int, n_modes: int = 3) -> Tuple[Opcode, Modes]:
        opcode_part = opcode % 100

        modes: List[int] = []
        opcode = opcode // 100

        for _ in range(n_modes):
            modes.append(opcode % 10)
            opcode = opcode // 10

        return Opcode(opcode_part), modes

    def add_input(self, input:int):
        self.inputs.append(input)

    def get_value(self, address: int, mode: int) -> int:
        if mode == ParameterMode.POSITION:
            return self.memory[self.memory[address]]
        elif mode == ParameterMode.IMMEDIATE:
            return self.memory[address]
        elif mode == ParameterMode.RELATIVE:
            return self.memory[self.memory[address] + self.relative_base]
        else:
            raise ValueError(f'Unknown parameter mode {mode}')

    def get_location(self, address: int, mode: int) -> int:
        if mode == ParameterMode.POSITION:
            return self.memory[address]
        elif mode == ParameterMode.RELATIVE:
            return self.memory[address] + self.relative_base
        else:
            raise ValueError(f'Unknown parameter mode {mode}')

    def run(self):
        while True:
            opcode, modes = self.parse_opcode(self.memory[self.address])
            if opcode == Opcode.ADD:
                p1 = self.get_value(self.address+1, modes[0])
                p2 = self.get_value(self.address+2, modes[1])
                p3 = self.get_location(self.address+3, modes[2])
                self.memory[p3] = p1 + p2
                self.address += 4
            elif opcode == Opcode.MULTIPLY:
                p1 = self.get_value(self.address+1, modes[0])
                p2 = self.get_value(self.address+2, modes[1])
                p3 = self.get_location(self.address+3, modes[2])
                self.memory[p3] = p1 * p2
                self.address += 4
            elif opcode == Opcode.INPUT:
                if len(self.inputs) < 1:
                    break
                else:
                    in_value = self.inputs.pop()
                    p1 = self.get_location(self.address+1, modes[0])
                    self.memory[p1] = in_value
                    self.address += 2
            elif opcode == Opcode.OUTPUT:
                p1 = self.get_value(self.address+1, modes[0])
                self.address += 2
                yield p1
            elif opcode == Opcode.JUMP_IF_TRUE:
                p1 = self.get_value(self.address+1, modes[0])
                p2 = self.get_value(self.address+2, modes[1])
                if p1 != 0:
                    self.address = p2
                else:
                    self.address += 3
            elif opcode == Opcode.JUMP_IF_FALSE:
                p1 = self.get_value(self.address+1, modes[0])
                p2 = self.get_value(self.address+2, modes[1])
                if p1 == 0:
                    self.address = p2
                else:
                    self.address += 3
            elif opcode == Opcode.LESS_THAN:
                p1 = self.get_value(self.address+1, modes[0])
                p2 = self.get_value(self.address+2, modes[1])
                p3 = self.get_location(self.address+3, modes[2])
                if p1 < p2:
                    self.memory[p3] = 1
                else:
                    self.memory[p3] = 0
                self.address += 4
            elif opcode == Opcode.EQUALS:
                p1 = self.get_value(self.address+1, modes[0])
                p2 = self.get_value(self.address+2, modes[1])
                p3 = self.get_location(self.address+3, modes[2])
                if p1 == p2:
                    self.memory[p3] = 1
                else:
                    self.memory[p3] = 0
                self.address += 4
            elif opcode == Opcode.RELATIVE_BASE_OFFSET:
                p1 = self.get_value(self.address+1, modes[0])
                self.relative_base += p1
                self.address += 2
            elif opcode == Opcode.HALT:
                break
            else:
                raise ValueError(f'Incorrect opcode {opcode}')
