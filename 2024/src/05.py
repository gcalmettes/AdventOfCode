from collections import defaultdict
from typing import List

with open("inputs/05.in")  as f:
    rules_data,pages_updates_data = f.read().strip().split("\n\n")

rules = defaultdict(list)
for rule in rules_data.split("\n"):
    a,b = rule.split("|")
    rules[int(a)].append(int(b))

pages_updates = []
for page_update in pages_updates_data.split("\n"):
    pages_updates.append([int(a) for a in page_update.split(",")])

def is_valid_ordering(page_update: List[int]) -> bool:
    for i,p in enumerate(page_update):
        rp = rules.get(p)
        if rp:
            # ensure all the dependent pages are after
            for dp in rp:
               if dp in page_update and page_update.index(dp) < i:
                  return False
    return True

def reorder(update: List[int]) -> List[int]:
    is_reordered = False
    ordered = []
    to_reorder = []
    while len(ordered) < len(update) and not is_reordered:
        tmp = ordered + to_reorder
        if not tmp:
            tmp = update
        assert len(tmp)==len(update)
        to_reorder = []
        for i,p in enumerate(tmp):
            rp = rules.get(p)
            if rp:
                for dp in rp:
                    if dp in tmp and tmp.index(dp) < i and dp not in to_reorder:
                        to_reorder.append(dp)
        ordered = [u for u in tmp if u not in to_reorder]
        assert (len(ordered) + len(to_reorder)) == len(update)
        if len(to_reorder) == 0:
            is_reordered = True

    return ordered


p1, p2 = 0,0
to_order = []
for update in pages_updates:
    if is_valid_ordering(update):
        mid = len(update)//2
        p1 += update[mid]
    else:
        reordered = reorder(update)
        mid = len(reordered)//2
        p2 += reordered[mid]

print(f"part1: {p1}")
print(f"part2: {p2}")
