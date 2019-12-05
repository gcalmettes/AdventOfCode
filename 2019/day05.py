"""
https://adventofcode.com/2019/day/2
"""

from typing import List, Tuple


Program = List[int]


def read_opcode(opcode: int) -> Tuple[int, List[int]]:
    opcode_ = opcode % 100

    mode_param1 = (opcode // 100) % 10
    mode_param2 = (opcode // 1000) % 10
    mode_param3 = (opcode // 10000) % 10

    return (opcode_, [mode_param1, mode_param2, mode_param3])


def run_program(program: Program, input: List[int]) -> List[int]:
    program = program[:]
    output = []

    pos = 0

    while program[pos] != 99:
        opcode, params = read_opcode(program[pos])

        # addition
        if opcode == 1:
            if params[0] == 0:
                # position mode
                value1 = program[program[pos + 1]]
            else:
                # immediate mode
                value1 = program[pos + 1]

            if params[1] == 0:
                # position mode
                value2 = program[program[pos + 2]]
            else:
                # immediate mode
                value2 = program[pos + 2]
            program[program[pos + 3]] = value1 + value2
            pos += 4 # n params

        # multiplication
        elif opcode == 2:
            if params[0] == 0:
                # position mode
                value1 = program[program[pos + 1]]
            else:
                # immediate mode
                value1 = program[pos + 1]

            if modes[1] == 0:
                # position mode
                value2 = program[program[pos + 2]]
            else:
                # immediate mode
                value2 = program[pos + 2]
            program[program[pos + 3]] = value1 * value2
            pos += 4

        # Take input and store
        elif opcode == 3:
            loc = program[pos + 1]
            input_value = input[0]
            input = input[1:]
            program[loc] = input_value
            pos += 2

        # Take the output
        elif opcode == 4:
            if modes[0] == 0:
                loc = program[pos + 1]
                value = program[loc]
            else:
                value = program[pos + 1]

            output.append(value)
            pos += 2

        # jump if true
        elif opcode == 5:
            if modes[0] == 0:
                value1 = program[program[pos + 1]]
            else:
                value1 = program[pos + 1]

            if modes[1] == 0:
                value2 = program[program[pos + 2]]
            else:
                value2 = program[pos + 2]

            if value1 != 0:
                pos = value2
            else:
                pos += 3

        # jump if false
        elif opcode == 6:
            if modes[0] == 0:
                value1 = program[program[pos + 1]]
            else:
                value1 = program[pos + 1]

            if modes[1] == 0:
                value2 = program[program[pos + 2]]
            else:
                value2 = program[pos + 2]

            if value1 == 0:
                pos = value2
            else:
                pos += 3

        # less than
        elif opcode == 7:
            if modes[0] == 0:
                value1 = program[program[pos + 1]]
            else:
                value1 = program[pos + 1]

            if modes[1] == 0:
                value2 = program[program[pos + 2]]
            else:
                value2 = program[pos + 2]

            if value1 < value2:
                program[program[pos + 3]] = 1
            else:
                program[program[pos + 3]] = 0
            pos += 4

        # equals
        elif opcode == 8:
            if modes[0] == 0:
                value1 = program[program[pos + 1]]
            else:
                value1 = program[pos + 1]

            if modes[1] == 0:
                value2 = program[program[pos + 2]]
            else:
                value2 = program[pos + 2]

            if value1 == value2:
                program[program[pos + 3]] = 1
            else:
                program[program[pos + 3]] = 0
            pos += 4

        else:
            raise ValueError(f"invalid opcode: {opcode}")

    return output



with open('day05_input.txt', 'r') as f:
    data = list(map(int, f.readline().split(',')))

part1 = run_program(data, [1])
print(f'part1: {part1}')

part2 = run_program(data, [5])
print(f'part2: {part2}')



