"""
https://adventofcode.com/2019/day/2
"""

from typing import List


Program = List[int]


def reset_program(program: Program) -> Program:
    program = program[:]
    program[1] = 12
    program[2] = 2
    return program


def read_instructions(program: Program, step: int) -> List[int]:
    return program[4 * step:4 * (step + 1)]


def run_step(program: Program, step: int) -> bool:
    instructions = read_instructions(program, step)
    if instructions[0] == 99:
        return False
    else:
        if instructions[0] == 1:
            program[instructions[3]] = program[instructions[1]] + program[instructions[2]]
        elif instructions[0] == 2:
            program[instructions[3]] = program[instructions[1]] * program[instructions[2]]
        else:
            raise RuntimeError(f'Unknown OPCODE: {instructions[0]}')
        return True


def run_program(program: Program) -> Program:
    step = -1
    go = True

    while go:
        step += 1
        go = run_step(program, step)

    return program


def check_input(program: Program, noun: int = 12, verb: int = 2) -> int:
    program = program[:]
    program[1] = noun
    program[2] = verb
    run_program(program)
    return program[0]


with open('day02_input.txt', 'r') as f:
    data = list(map(int, f.readline().split(',')))

# part 1
print('------- part 1 -------')

assert run_program([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]) == [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
assert run_program([1, 0, 0, 0, 99]) == [2, 0, 0, 0, 99]
assert run_program([2, 3, 0, 3, 99]) == [2, 3, 0, 6, 99]
assert run_program([2, 4, 4, 5, 99, 0]) == [2, 4, 4, 5, 99, 9801]
assert run_program([1, 1, 1, 4, 99, 5, 6, 0, 99]) == [30, 1, 1, 4, 2, 5, 6, 0, 99]

part1_data = reset_program(data)
print(run_program(part1_data)[0])


# part 2
print('------- part 2 -------')

target = 19690720

for noun in range(100):
    for verb in range(100):
        output = check_input(data, noun, verb)
        if output == target:
            print(100 * noun + verb)
            break



