from typing import Dict, NamedTuple, List
from collections import defaultdict
import math


class Element(NamedTuple):
    name: str
    amount: int

    @staticmethod
    def parse_element(element: str) -> 'Element':
        amount, name = element.split(' ')
        return Element(name, int(amount))


class Reaction(NamedTuple):
    inputs: List[Element]
    output: Element

    @staticmethod
    def parse_reaction(reaction: str) -> 'Reaction':
        inputs, output = reaction.strip().split(' => ')
        inputs = [Element.parse_element(element) for element in inputs.split(', ')]
        output = Element.parse_element(output)
        return Reaction(inputs, output)


def parse(inputs: str) -> Dict:
    reactions = [Reaction.parse_reaction(line) for line in inputs.split('\n')]
    return reactions


def calculate_ORE_needed(inputs: str, amount: int=1) -> int:
    reactions = parse(inputs)

    recipes = {reaction.output.name: {
                    'amount': reaction.output.amount, 
                    'made_from': reaction.inputs}
                    for reaction in reactions
                }

    requirements = defaultdict(int, {'FUEL': amount})
    ORE_quantity = 0

    def done() -> bool:
        return all(amount <= 0 for amount in requirements.values())

    while not done():
        # filter what is still needed
        needed = {name: amount for name, amount in requirements.items() if amount > 0}
        # get what we need for that
        for needed_element, needed_amount in needed.items():
            reagents = recipes[needed_element]['made_from']
            produced_amount = recipes[needed_element]['amount']
            # number of times we need reaction
            n_times = math.ceil(needed_amount / produced_amount)

            # make reaction
            requirements[needed_element] -= n_times * produced_amount

            # update what we used
            for i, reagent in enumerate(reagents):
                if reagent.name == 'ORE':
                    ORE_quantity += reagent.amount * n_times
                else:
                    requirements[reagent.name] += reagent.amount * n_times


    return ORE_quantity

t1 = '''10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL'''

t2 = '''9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL'''

t3 = '''157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT'''

assert calculate_ORE_needed(t1) == 31
assert calculate_ORE_needed(t2) == 165
assert calculate_ORE_needed(t3) == 13312


def get_maximum_fuel_with(ore_available: int, inputs: str) -> int:
    
    fuel_low = 1
    ore_low = calculate_ORE_needed(inputs, fuel_low)

    fuel_high = ore_available // ore_low
    ore_high = calculate_ORE_needed(inputs, fuel_high)

    # calibrate range of fuel to examine
    while ore_high < ore_available:
        fuel_low = fuel_high
        ore_low = calculate_ORE_needed(inputs, fuel_low)

        fuel_high = fuel_high * 2
        ore_high = calculate_ORE_needed(inputs, fuel_high)

    while (fuel_low + 1) < fuel_high:
        fuel_middle = (fuel_low + fuel_high) // 2
        ore_middle = calculate_ORE_needed(inputs, fuel_middle)

        if ore_middle <= ore_available:
            fuel_low, ore_low = fuel_middle, ore_middle
        
        else:
            fuel_high, ore_high = fuel_middle, ore_middle

    return fuel_middle

with open('day14_input.txt') as f:
    data = f.read()

part1 = calculate_ORE_needed(data)
print(f'part 1: {part1}')

ore_available = 1_000_000_000_000
part2 = get_maximum_fuel_with(ore_available, data)
print(f'part 2: {part2}')

