
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

Modes = List[int]
Program = defaultdict(int)

def listToDict(p: List[int]) -> Program:
    program = defaultdict(int)
    program.update({i: val for i, val in enumerate(p)})
    return program

def parse_opcode(opcode: int, n_modes: int = 3) -> Tuple[Opcode, Modes]:
    opcode_part = opcode % 100

    modes: List[int] = []
    opcode = opcode // 100

    for _ in range(n_modes):
        modes.append(opcode % 10)
        opcode = opcode // 10

    return Opcode(opcode_part), modes

def get_value(program: Program, pos: int, mode: int, relative_base: int=0, write=False) -> int:
    if mode == ParameterMode.POSITION:
        if write:
            return program[pos]
        else:
            return program[program[pos]]
    elif mode == ParameterMode.IMMEDIATE:
        return program[pos]
    elif mode == ParameterMode.RELATIVE:
        if write:
            return program[pos] + relative_base
        else:
            return program[program[pos] + relative_base]
    else:
        raise ValueError(f'Unknown parameter mode {mode}')


def run_intcode(program: Program, inputs=[0], cursor: int=0, r_base_offset: int=0) -> Tuple[int, int]:

    output = []

    while True:
        opcode, modes = parse_opcode(program[cursor])

        if opcode == Opcode.ADD:
            p1 = get_value(program, cursor+1, modes[0], r_base_offset)
            p2 = get_value(program, cursor+2, modes[1], r_base_offset)
            p3 = get_value(program, cursor+3, modes[2], r_base_offset, True)
            program[p3] = p1 + p2
            cursor += 4
        elif opcode == Opcode.MULTIPLY:
            p1 = get_value(program, cursor+1, modes[0], r_base_offset)
            p2 = get_value(program, cursor+2, modes[1], r_base_offset)
            p3 = get_value(program, cursor+3, modes[2], r_base_offset, True)
            program[p3] = p1 * p2
            cursor += 4
        elif opcode == Opcode.INPUT:
            in_value = inputs.pop()
            p1 = get_value(program, cursor+1, modes[0], r_base_offset, True)
            program[p1] = in_value
            cursor += 2
        elif opcode == Opcode.OUTPUT:
            p1 = get_value(program, cursor+1, modes[0], r_base_offset)
            cursor += 2
            output += [p1]
        elif opcode == Opcode.JUMP_IF_TRUE:
            p1 = get_value(program, cursor+1, modes[0], r_base_offset)
            p2 = get_value(program, cursor+2, modes[1], r_base_offset)
            if p1 != 0:
                cursor = p2
            else:
                cursor += 3
        elif opcode == Opcode.JUMP_IF_FALSE:
            p1 = get_value(program, cursor+1, modes[0], r_base_offset)
            p2 = get_value(program, cursor+2, modes[1], r_base_offset)
            if p1 == 0:
                cursor = p2
            else:
                cursor += 3
        elif opcode == Opcode.LESS_THAN:
            p1 = get_value(program, cursor+1, modes[0], r_base_offset)
            p2 = get_value(program, cursor+2, modes[1], r_base_offset)
            p3 = get_value(program, cursor+3, modes[2], r_base_offset, True)
            if p1 < p2:
                program[p3] = 1
            else:
                program[p3] = 0
            cursor += 4
        elif opcode == Opcode.EQUALS:
            p1 = get_value(program, cursor+1, modes[0], r_base_offset)
            p2 = get_value(program, cursor+2, modes[1], r_base_offset)
            p3 = get_value(program, cursor+3, modes[2], r_base_offset, True)
            if p1 == p2:
                program[p3] = 1
            else:
                program[p3] = 0
            cursor += 4
        elif opcode == Opcode.RELATIVE_BASE_OFFSET:
            p1 = get_value(program, cursor+1, modes[0], r_base_offset)
            r_base_offset += p1
            cursor += 2
        elif opcode == Opcode.HALT:
            break
        else:
            raise ValueError(f'Incorrect opcode {opcode}')

    return output
