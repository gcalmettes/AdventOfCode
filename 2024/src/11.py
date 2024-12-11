input = "6563348 67 395 0 6 4425 89567 739318"

stones = [int(s) for s in input.split()]

cache = {}
def blink(stone, step):
    if (stone, step) in cache:
        # if we've seen the pattern already, return it
        return cache[(stone, step)]
    if step==0:
        # no step left nothing happens to the stone
        count = 1
    elif stone==0:
        count = blink(1, step-1)
    elif len(str(stone)) % 2 == 0:
        # split the stone equally
        stone_str = str(stone)
        mid = len(stone_str) // 2
        split1, split2 = stone_str[:mid], stone_str[mid:]
        split1, split2 = int(split1), int(split2)
        count = blink(split1, step-1) + blink(split2, step-1)
    else:
        count = blink(stone * 2024, step-1)
    cache[(stone, step)] = count
    return count


p1 = sum(blink(stone, 25) for stone in stones)
p2 = sum(blink(stone, 75) for stone in stones)


print(f"part1: {p1}")
print(f"part2: {p2}")
