"""
https://adventofcode.com/2019/day/6
"""

from typing import List, Dict, NamedTuple


class Node(NamedTuple):
    link_to: str
    name: str


def parse(node: str) -> Node:
    return Node(*node.split(')'))


def get_nodes(graph: List[str]) -> Dict[str, str]:
    nodes = dict()
    for text in graph:
        node = parse(text)
        nodes[node.name] = node.link_to
    return nodes


def count_connections(nodes: Dict[str, str]) -> int:
    all_linkers = list(nodes.keys())
    count = 0
    for node in all_linkers:
        connections = 0
        current = node
        # print(f'-- starting current: {current}')
        while True:
            try:
                current = nodes[current]
                # print(f'   > linked to: {current}')
                connections += 1
            except KeyError:
                count += connections
                # print(f'   ## {connections}')
                break
    return count


def count_orbital_transfers(nodes: Dict[str, str],
                            start1='YOU', start2='SAN') -> int:
    current_start1 = start1
    current_start2 = start2
    from_start1 = []
    from_start2 = []

    c1_in_s2 = False
    c2_in_s1 = False

    count = 0
    while not (c1_in_s2 or c2_in_s1):
        current_start1 = nodes[current_start1]
        current_start2 = nodes[current_start2]

        from_start1.append(current_start1)
        from_start2.append(current_start2)

        c1_in_s2 = current_start1 in from_start2
        c2_in_s1 = current_start2 in from_start1

        count += 1

    if c1_in_s2:
        count = len(from_start1) + from_start2.index(current_start1) - 1
    else:
        count = len(from_start2) + from_start1.index(current_start2) - 1

    return count


test1 = '''COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L'''

test2 = '''COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN'''

test1 = get_nodes(test1.split('\n'))
test2 = get_nodes(test2.split('\n'))

assert count_connections(test1) == 42
assert count_orbital_transfers(test2) == 4


with open('day06_input.txt', 'r') as f:
    data = [line.strip() for line in f.readlines()]

nodes = get_nodes(data)

part1 = count_connections(nodes)
print(f'part1: {part1}')

part2 = count_orbital_transfers(nodes)
print(f'part2: {part2}')
