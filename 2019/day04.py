"""
https://adventofcode.com/2019/day/4
"""

from typing import List
from collections import Counter
from itertools import compress

INPUT = '128392-643281'
LOW, HIGH = map(int, INPUT.split('-'))

def convert_to_list(digit: int) -> List[int]:
    return list(map(int, list(str(digit))))

def is_increasing(digit: List[int]) -> bool:
    return all(b >= a for a, b in zip(digit[:-1], digit[1:]))

def has_adjacent_same(digit: List[int]) -> bool:
    return any(a == b for a, b in zip(digit[:-1], digit[1:]))

def has_unique_double(digit: List[int]) -> bool:
    is_a_double = [a == b for a, b in zip(digit[:-1], digit[1:])]
    all_double = list(compress(zip(digit[:-1], digit[1:]), is_a_double))
    count = Counter(all_double)
    return any(v == 1 for v in count.values())

def count_valid_passwords1(low: int, high: int) -> int:
    count = 0
    for i in range(low, high+1):
        to_check = convert_to_list(i)
        if has_adjacent_same(to_check) and is_increasing(to_check):
            count += 1
    return count

def count_valid_passwords2(low: int, high: int) -> int:
    count = 0
    for i in range(low, high+1):
        to_check = convert_to_list(i)
        if has_adjacent_same(to_check) and is_increasing(to_check) and has_unique_double(to_check):
            count += 1
    return count


part1 = count_valid_passwords1(LOW, HIGH)
print('part 1:', part1)

part2 = count_valid_passwords2(LOW, HIGH)
print('part 2:', part2)