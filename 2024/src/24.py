with open("inputs/24.in")  as f:
    input = f.read().strip()

values, conn = input.split("\n\n")
values = {k: int(v) for line in values.strip().split("\n") for k,v in [line.split(":")]}
connections = []
max_z = "z00"
for c in conn.split("\n"):
    parts, res = c.split(" -> ")
    a,op,b = parts.split()
    connections.append((a,op,b,res))
    if res[0] == "z" and int(res[1:]) > int(max_z[1:]):
        max_z = res

wrong = set()
for a, op, b, res in connections:
    if res[0] == "z" and op != "XOR" and res != max_z:
        wrong.add(res)
    if (
        op == "XOR"
        and res[0] not in ["x", "y", "z"]
        and a[0] not in ["x", "y", "z"]
        and b[0] not in ["x", "y", "z"]
    ):
        wrong.add(res)
    if op == "AND" and "x00" not in [a, b]:
        for a2, op2, b2, res2 in connections:
            if (res == a2 or res == b2) and op2 != "OR":
                wrong.add(res)
    if op == "XOR":
        for a2, op2, b2, res2 in connections:
            if (res == a2 or res == b2) and op2 == "OR":
                wrong.add(res)

while len(connections):
    connections_left = []
    has_z = False
    for (a,op,b,res) in connections:
        if values.get(a) is not None and values.get(b) is not None:
            a,b = values.get(a), values.get(b)
            if op == "OR":
                values[res] = int(bool(a) or bool(b))
            elif op == "AND":
                values[res] = int(bool(a) and bool(b))
            elif op == "XOR":
                values[res] = int(bool(a) ^ bool(b))
            else:
                raise("unknown op")
        else:
            if not has_z and (a.startswith("z") or b.startswith("z") or res.startswith("z")):
                has_z = True
            connections_left.append((a,op,b,res))
    connections = connections_left
    if not has_z:
        break


outputs = [(k, v) for k,v in values.items() if k.startswith("z")]
outputs = "".join(str(b) for a,b in sorted(outputs, key=lambda x: x[0], reverse=True))

p1 = int(outputs, 2)
p2 = ",".join(sorted(wrong))

print(f"part1: {p1}")
print(f"part2: {p2}")
