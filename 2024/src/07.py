with open("inputs/07.in")  as f:
    input = f.read().strip().split("\n")

def is_calibrated(target, nums, p2=False):
    if len(nums) == 1:
        return nums[0]==target
    acc,left = nums[0],nums[1:]
    mul = is_calibrated(target, [acc*left[0]] + left[1:], p2)
    add = is_calibrated(target, [acc+left[0]] + left[1:], p2)
    if p2:
        concat = is_calibrated(target, [int(f"{acc}{left[0]}")]+ left[1:], True)
        return mul or add or concat
    return mul or add

equations = [(int(target), [int(n) for n in nums.strip().split()]) for line in input for target,nums in [line.split(":")]]

p1 = p2 = 0
for target,nums in equations:
    if is_calibrated(target, nums):
        p1 += target
    if is_calibrated(target, nums, True):
        p2 += target

print(f"part1: {p1}")
print(f"part2: {p2}")
