"""
https://adventofcode.com/2019/day/1
"""

with open('day01_input.txt', 'r') as f:
    modules = [int(line.strip()) for line in f]

# part 1
print('------- part 1 -------')


def compute_fuel_required(rocket_mass: int) -> int:
    return int(rocket_mass / 3) - 2


assert compute_fuel_required(12) == 2
assert compute_fuel_required(14) == 2
assert compute_fuel_required(1969) == 654
assert compute_fuel_required(100756) == 33583

fuel_loads = [compute_fuel_required(mass) for mass in modules]
print(sum(fuel_loads))

# part 2
print('------- part 2 -------')


def compute_extended_fuel_required(mass: int, total: int=0) -> int:
    fuel = compute_fuel_required(mass)
    total += fuel
    if compute_fuel_required(fuel) <= 0:
        return total
    else:
        return compute_extended_fuel_required(fuel, total)


assert compute_extended_fuel_required(14) == 2
assert compute_extended_fuel_required(1969) == 966
assert compute_extended_fuel_required(100756) == 50346

loaded_modules = [compute_extended_fuel_required(mass) for mass in modules]
print(sum(loaded_modules))
