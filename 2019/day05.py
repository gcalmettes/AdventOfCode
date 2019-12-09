"""
https://adventofcode.com/2019/day/5
"""

from intcode import run_intcode



with open('day05_input.txt', 'r') as f:
    data = list(map(int, f.readline().split(',')))


part1 = run_intcode(data, inputs=[1])[1][-1]
print(f'part1: {part1}')

part2 = run_intcode(data, inputs=[5])[1][-1]
print(f'part2: {part2}')