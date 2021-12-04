"""
https://adventofcode.com/2019/day/16
"""

from typing import List, Iterator
import itertools

def pattern(output_element: int) -> Iterator[int]:
   while True:
        for _ in range(output_element):
           yield 0
        for _ in range(output_element):
           yield 1
        for _ in range(output_element):
           yield 0
        for _ in range(output_element):
           yield -1


def ones_digit(n: int) -> int:
    if n > 0:
        return n % 10
    else:
        return (-n) % 10


def fft_phase(numbers: List[int]) -> List[int]:
    output = []
    n = len(numbers)
    for i in range(n):
        pat = pattern(i + 1)
        next(pat) # skip first pattern
        
        values = list(zip(pat, numbers))
        #print(values)
        total = sum(p * n for p, n in values)
        #print(total)
        output.append(ones_digit(total))
    return output


def apply_fft(numbers: List[int], n: int) -> int:
    for _ in range(n):
        numbers = fft_phase(numbers)
    return ''.join(map(str, numbers[:8]))


with open('day16_input.txt') as f:
    data = [int(x) for x in f.read().strip()]

part1 = apply_fft(data, 100)
print(f'part 1: {part1}')

### part 2

def part2(raw: str) -> List[int]:
    offset = int(raw[:7])
    numbers = [int(c) for c in raw] * 10_000

    assert offset > len(numbers) // 2

    # pattern is tail([0] * n, [1] * n, [0] * n, [-1] * n)
    # in particular, pattern is 0 up until place n
    # in particular, if n >= len(numbers) // 2, pattern is just 1 starting at n until the end

    # that means we only need to sum up until the end
    for _ in range(100):

        # last position
        pos = len(numbers) - 1
        total = 0

        while pos >= offset:
            total += numbers[pos]
            numbers[pos] = ones_digit(total)
            pos -= 1


    return numbers[offset:offset+8]

with open('day16_input.txt') as f:
    print(part2(f.read().strip()))