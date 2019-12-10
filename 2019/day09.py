"""
https://adventofcode.com/2019/day/9
"""

from intcode_final import run_intcode, listToDict

p = listToDict([109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99])
assert f'{list(p.values())}' == f'{run_intcode(p, inputs=[])}'

assert len(str(run_intcode([1102,34915192,34915192,7,4,7,99,0], inputs=[])[0])) == 16

p = listToDict([104,1125899906842624,99])
assert run_intcode([104,1125899906842624,99], inputs=[]) == [p[1]]


with open('day09_input.txt') as f:
    program = listToDict([int(x) for x in f.read().strip().split(',')])

part1 = run_intcode(program, inputs=[1])
print(f'part 1: {part1}')

part2 = run_intcode(program, inputs=[2])
print(f'part 2: {part2}')