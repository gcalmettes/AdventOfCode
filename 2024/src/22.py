from collections import defaultdict

with open("inputs/22.in")  as f:
    input = f.read().strip()

def mix(secret: int, n: int) -> int:
    return secret ^ n

def prune(secret:int) -> int:
    return secret % 16777216

def get_next(secret:int) -> int:
    d = secret * 64
    secret = mix(secret, d)
    secret = prune(secret)
    d = secret // 32
    secret = mix(secret, d)
    secret = prune(secret)
    d = secret * 2048
    secret = mix(secret, d)
    secret = prune(secret)
    return secret

p1 = 0
seqs = defaultdict(list)
patterns = defaultdict(int)
for i,line in enumerate(input.split("\n")):
    secret = int(line)
    seqs[i].append((secret%10, 0))
    seen = set()
    for j in range(2000):
        secret = get_next(secret)
        last = secret%10
        diff = last - seqs[i][-1][0]
        seqs[i].append((last, diff))
        if j>1:
            pattern = tuple([a[1] for a in seqs[i][-4:]])
            if pattern not in seen:
                patterns[pattern] += last
                seen.add(pattern)
    p1 += secret

p2 = max(patterns.values())

print(f"part1: {p1}")
print(f"part2: {p2}")
