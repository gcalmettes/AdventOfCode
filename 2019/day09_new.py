"""
https://adventofcode.com/2019/day/9
"""

from intcodeComputer import IntcodeComputer

p = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]
cpu = IntcodeComputer(p)
res = [i for i in cpu.run()]
assert f'{p}' == f'{res}'

p = [1102,34915192,34915192,7,4,7,99,0]
cpu = IntcodeComputer(p)
res = [i for i in cpu.run()]
assert len(str(res[0])) == 16

p = [104,1125899906842624,99]
cpu = IntcodeComputer(p)
res = [i for i in cpu.run()]
assert res == [p[1]]

with open('day09_input.txt') as f:
    program = [int(x) for x in f.read().strip().split(',')]

cpu = IntcodeComputer(program, inputs=[1])
part1 = next(cpu.run())
print(f'part 1: {part1}')

cpu = IntcodeComputer(program, inputs=[2])
part2 = next(cpu.run())
print(f'part 2: {part2}')