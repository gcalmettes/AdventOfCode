import re

with open("inputs/03.in")  as f:
    input = f.read()

def remove_dont_sequences(line):
    splitted = []
    while len(line) > 1:
        parts = line.split("do", 1)
        if len(parts) == 2:
            a, b = parts
            splitted += [a]
            if b.startswith("n't()"):
                splitted += ["don't()"]
                line = b[5:]
            elif b.startswith("()"):
                splitted += ["do()"]
                line = b[2:]
            else:
                line = b
        else:
            splitted += parts
            line = ""
    cleaned = ""
    add = True
    for chunk in splitted:
        if add and chunk not in ["don't()", "do()"]:
            cleaned += chunk
        elif chunk == "don't()":
            add = False
        elif chunk == "do()":
            add = True
    return cleaned


def compute(input):
    total = 0
    matches = re.findall(r"mul\([0-9]+,[0-9]+\)", input)
    for m in matches:
        l,r = m.split(",")
        l = int(l.removeprefix("mul("))
        r = int(r[:-1])
        total += l*r
    return total

p1, p2 = 0, 0

p1 = compute(input)

matches = re.findall(r"mul\([0-9]+,[0-9]+\)", input)
for m in matches:
    l,r = m.split(",")
    l = int(l.removeprefix("mul("))
    r = int(r[:-1])
    p1 += l*r

cleaned = remove_dont_sequences(input)

p1 = compute(input)
p2 = compute(cleaned)
# matches = re.findall(r"mul\([0-9]+,[0-9]+\)", cleaned_line)
# for m in matches:
#     l,r = m.split(",")
#     l = int(l.removeprefix("mul("))
#     r = int(r[:-1])
#     p2 += l*r

print(f"part1: {p1}")
print(f"part2: {p2}")
