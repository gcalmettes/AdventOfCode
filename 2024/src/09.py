from collections import defaultdict

with open("inputs/09.in")  as f:
    input = f.read().strip()

disk = dict()

free = defaultdict(set) # size free spot: {set of pos}
# for p2
files_sizes = dict() # id: size
files_pos = dict() # id: position

cursor = 0
file_id = 0
for i,c in enumerate(input):
    n = int(c)
    if i%2 == 0: # file
        files_sizes[file_id] = n
        files_pos[file_id] = cursor
        for _ in range(n):
            disk[cursor] = file_id
            cursor += 1
        file_id += 1
    else: # free space
        if n > 0:
            free[n].add(cursor)
        cursor += n

def checksum(disk: dict) -> int:
    s = 0
    for k,v in disk.items():
        s += k*v
    return s

# part 1
pos = sorted(disk.keys(), reverse=True)
cursor = 0
for i in range(len(pos)):
    if disk.get(i) is None:
        disk[i] = disk.get(pos[cursor])
        del disk[pos[cursor]]
        cursor += 1

p1 = checksum(disk)

# part 2
files_ids = sorted(files_sizes.keys(), reverse=True)
max_free = max(free.keys())

for id in files_ids:
   file_size = files_sizes[id]
   would_fit = dict() # pos: size
   for s in range(file_size, max_free + 1):
       s_sizes = free.get(s)
       if s_sizes:
           would_fit = {**would_fit, **{v: s for v in s_sizes}}
   if would_fit: # large enough free spot available
       new_pos = min(would_fit.keys())
       if new_pos < files_pos[id]: # make sure new spot is before actual position
           files_pos[id] = new_pos
           # update pos and size of available spot
           filled_initial_size = would_fit[new_pos]
           # free[filled_initial_size] = [s for s in free[filled_initial_size] if s != new_pos]
           free[filled_initial_size].remove(new_pos)
           if filled_initial_size > file_size: # need to update free spots available
               remaining =  filled_initial_size - file_size
               free[remaining].add(new_pos+file_size)

# construct disk based on files pos
disk2 = dict()
for id,pos in files_pos.items():
    size = files_sizes[id]
    for i in range(size):
        disk2[pos+i] = id

p2 = checksum(disk2)

print(f"part1: {p1}")
print(f"part2: {p2}")
