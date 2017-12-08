"""
http://adventofcode.com/2017/day/7
"""

import re
from collections import defaultdict


ex = """root (10) -> aa, bb, cc
aa (21) -> aaa, aaaa, aaaaa
bb (320) -> bbb, bbbb, bbbbb
cc (320) -> ccc, cccc, ccccc
aaa (26) -> ee, eee, eeee
aaaa (146) -> ff, fff, ffff
aaaaa (170) -> gg, ggg, gggg
bbb (16) -> hh, hhh, hhhh
bbbb (40) -> ii, iii, iii
bbbbb (73) -> jj, jjj, jjjj
ccc (10) -> kk, kkk, kkkk
cccc (40) -> ll, lll, llll
ccccc (70) -> mm, mmm, mmmm
ee (58)
eee (58)
eeee (58)
ff (18)
fff (18)
ffff (18)
gg (10)
ggg (10)
gggg (10)
hh (28)
hhh (28)
hhhh (28)
ii (20)
iii (20)
iiii (20)
jj (9)
jjj (9)
jjjj (9)
kk (30)
kkk (30)
kkkk (30)
ll (20)
lll (20)
llll (20)
mm (10)
mmm (10)
mmmm (10)
"""


# part 1
def getBase(lines):
    regex = "[a-z]*[a-z]"
    toMatch = re.compile(regex)

    parents = []
    childs = []

    for line in lines:
        towers = toMatch.findall(line)
        if len(towers)>1: #if towers support other towers
            parents.append(towers[0])
            childs.extend(towers[1:])

    return [parent for parent in parents if parent not in childs][0]




# part 2
def getWeigths(lines):
    """
    Get all the towers and their respective weight
    """
    regexName = "[a-z]*[a-z]"
    regexWeight = "[0-9]*[0-9]"
    toMatchName = re.compile(regexName)
    toMatchWeight = re.compile(regexWeight)

    structures = {}
    for line in lines:
        name = toMatchName.findall(line)[0]
        weight = int(toMatchWeight.findall(line)[0])
        structures[name] = weight

    return structures


def getChilds(lines):
    regex = "[a-z]*[a-z]"
    toMatch = re.compile(regex)

    network = {}#defaultdict(int)

    for line in lines:
        towers = toMatch.findall(line)
        if len(towers)>1: #if towers support other towers
            network[towers[0]] = towers[1:]

    return network

def getNetworkLevels(s):
    parentNetwork = getChilds(s)
    base = [getBase(s)]

    fullNetwork = {0: base}

    i = 1

    valid = True
    while valid:
        childs = [child for parent in base if parent in parentNetwork.keys() for child in parentNetwork[parent]]
        if len(childs)==0:
            valid=False
            break
        fullNetwork[i] = childs
        base = childs
        i+=1

    return fullNetwork


def calculateBalance(s):
    allWeights = getWeigths(s)
    parentNetwork = getChilds(s)
    networkLevels = getNetworkLevels(s)

    levels = sorted([int(key) for key in networkLevels.keys()])

    i = len(levels)-2 #second to last index
    if i<0:
        return "No network"
    else:
        while i>=0:
            for parent in networkLevels[i]:
                if parent not in parentNetwork.keys():
                    continue
                childsWeights = [allWeights[child] for child in parentNetwork[parent]]
                if len(set(childsWeights))>1:
                    initialWeights = getWeigths(s)
                    return [{"name": child, "initialWeight": initialWeights[child], "weightWithChild": allWeights[child]} for child in parentNetwork[parent]]
                else:
                    allWeights[parent] += sum(childsWeights)

            i -=1


def getCorrectedWeigth(s):
    unbalancedNodes = calculateBalance(s)
    unbalancedWeights = set([node["weightWithChild"] for node in unbalancedNodes])

    weightToAdjust = [weight for weight in unbalancedWeights if len([node["weightWithChild"] for node in unbalancedNodes if node["weightWithChild"]==weight])==1][0]
    diffToAdjust = weightToAdjust - [weight for weight in unbalancedWeights if weight != weightToAdjust][0]

    nodeToAdjust = [node for node in unbalancedNodes if node["weightWithChild"] == weightToAdjust][0]

    adjustedWeight = nodeToAdjust["initialWeight"] - diffToAdjust

    return adjustedWeight


if __name__ == "__main__":
	with open("day07_input.txt", "r") as f:
		INPUT = [line.strip() for line in f]
	# part 1
	print(getBase(INPUT))
	# part 2
	print(getCorrectedWeigth(INPUT))

