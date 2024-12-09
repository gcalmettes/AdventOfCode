with open("inputs/09.in")  as f:
    input = f.read().strip()

disk = dict()

cursor = 0
file_id = 0
for i,c in enumerate(input):
    n = int(c)
    if i%2 == 0: # file
        for _ in range(n):
            disk[cursor] = file_id
            cursor += 1
        file_id += 1
    else: # free space
        cursor += n
        # for _ in range(n):
        #     cursor += 1

pos = sorted(disk.keys(), reverse=True)

cursor = 0
for i in range(len(pos)):
    if disk.get(i) is None:
        disk[i] = disk.get(pos[cursor])
        del disk[pos[cursor]]
        cursor += 1

p1 = 0
for k,v in disk.items():
   p1 += k*v

print(f"part1: {p1}")
