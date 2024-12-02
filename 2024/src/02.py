with open("inputs/02.in")  as f:
    data = f.readlines()

data = [[int(n.strip()) for n in line.split()] for line in data if data]

def is_valid(arr):
    ordered = (arr == sorted(arr) or arr == sorted(arr, reverse=True))
    for i in range(len(arr)-1):
        if abs(arr[i+1] - arr[i]) < 1 or abs(arr[i+1] - arr[i]) > 3:
            return False
    return ordered

p1, p2 = 0, 0

for arr in data:
    if is_valid(arr):
        p1 +=1

    for i in range(len(arr)):
        sub_arr = arr[:i] + arr[i+1:]
        if is_valid(sub_arr):
            p2 += 1
            break

print(f"part1: {p1}")
print(f"part2: {p2}")
