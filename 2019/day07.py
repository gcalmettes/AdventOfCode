"""
https://adventofcode.com/2019/day/7
"""

from typing import List
from itertools import permutations

# import our intcode programme
from intcode import run_intcode


Program = List[int]


def run_amplifier_sequence(program: Program, phases: List[int]) -> int:

    current_input = 0

    for i in range(len(phases)):
        output = run_intcode(program, inputs=[phases[i], current_input])[0]
        current_input = output

    return output


def run_amplifier_loop(program: Program, phases: List) -> int:

    current_programs = [program[:] for _ in range(len(phases))]
    current_cursors = [0 for _ in range(len(phases))]

    current_input = 0
    current_non_none_input =  None
    n_halted = 0
    i = 0 

    while n_halted < len(phases):
        amp_program = current_programs[i % len(phases)]
        amp_cursor = current_cursors[i % len(phases)]
        if i < len(phases):
            inputs = [phases[i % len(phases)], current_input]
        else:
            inputs = [current_input]
        output, cursor = run_intcode(amp_program, cursor=amp_cursor, inputs=inputs)
        if output is None:
            n_halted += 1
        else:
            current_non_none_input =  output
        current_input = output

        current_cursors[i % len(phases)] = cursor
        i+= 1

    return current_non_none_input


assert run_amplifier_sequence(
    [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0],
    [4,3,2,1,0]) == 43210

assert run_amplifier_sequence(
    [3,23,3,24,1002,24,10,24,1002,23,-1,23,
     101,5,23,23,1,24,23,23,4,23,99,0,0],
    [0, 1, 2, 3, 4]) == 54321

assert run_amplifier_sequence(
    [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
     1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0],
    [1, 0, 4, 3, 2]) == 65210

assert run_amplifier_loop([3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5], [9,8,7,6,5]) == 139629729

assert run_amplifier_loop([3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10], [9,7,8,5,6]) == 18216


with open('day07_input.txt') as f:
    program = [int(x) for x in f.read().strip().split(',')]



part1 = max(run_amplifier_sequence(program, phases)
            for phases in permutations(range(5)))
print(f'part1: {part1}')


part2 = max(run_amplifier_loop(program, phases)
            for phases in permutations(range(5, 10)))
print(f'part2: {part2}')
