with open("inputs/19.in")  as f:
    input = f.read().strip()

TOWELS, PATTERNS = input.split("\n\n")

TOWELS = set(t.strip() for t in TOWELS.split(","))
PATTERNS = [p for p in PATTERNS.split("\n")]

p1 = p2 =0
SEEN = dict()

def number_of_valid(towels, pattern):
    if pattern in SEEN:
        # no need to compute again
        return SEEN[pattern]
    n = 0
    if not pattern:
        # we went successfully throught all the pattern to match
        n = 1
    for t in towels:
        # go through all the possible starting ways of starting the pattern
        if pattern.startswith(t):
            n += number_of_valid(towels, pattern[len(t):])
    SEEN[pattern] = n
    return n

for pattern in PATTERNS:
    n_valid = number_of_valid(TOWELS, pattern)
    if n_valid > 0:
        p1 += 1
    p2 += n_valid

print(f"part1: {p1}")
print(f"part2: {p2}")
