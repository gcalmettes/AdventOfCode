"""
http://adventofcode.com/2017/day/18
"""

from typing import NamedTuple, List, Dict
import functools

class Component(NamedTuple):
    name: str
    score: int
    toMatch: List[int]

def parseComponent(component: str) -> Component:
    a,b = list(map(lambda x: int(x), component.split("/")))
    return Component(component, a+b, [a, b])

def parseAllComponents(components: List[str]) -> List[Component]:
    return [parseComponent(component) for component in components]

def findMatch(bridge: List[Component], others: List[Component]) -> List[Component]:
    lastComponent = bridge[-1]
    matches = [component for component in others if component.name not in [c.name for c in bridge] and ((component.toMatch[0] in lastComponent.toMatch) or (component.toMatch[1] in lastComponent.toMatch))]
    return matches

def addToBridge(bridge: List[Component], component: Component) -> List[Component]:
    lastComponent = bridge[-1]
    newBridge = bridge[:-1]
    newBridge.append(Component(lastComponent.name, lastComponent.score, []))
    idxMatch = [x in lastComponent.toMatch for x in component.toMatch].index(True)
    remainingPort = [component.toMatch[x] for x in range(2) if x != idxMatch]
    newBridge.append(Component(component.name, component.score, remainingPort))
    return newBridge

def getAllBridges(startingComponent: Component, components: List[Component]) -> List[List[List[Component]]]:
    matches = findMatch([startingComponent], components)

    remainingMatches = False
    if len(matches) > 0:
        remainingMatches = True
        bridges = [[addToBridge([startingComponent], m) for m in matches]]
    else:
        bridges = [[startingComponent]]
    
    while remainingMatches:
        # print([len(findMatch(bridge, components))>0 for bridge in bridges[-1]])
        
        
        # print(remainingMatches)
        newBridges = []
        for bridge in bridges[-1]:
            extent = [addToBridge(bridge, component) for component in findMatch(bridge, components) if len(findMatch(bridge, components))>0]
            newBridges = newBridges + extent
        bridges.append(newBridges)
        remainingMatches = any([len(findMatch(bridge, components))>0 for bridge in bridges[-1]])
    return bridges

def getStartingComponents(components: List[Component]) -> List[Component]:
    return [Component(component.name, component.score, [x for x in component.toMatch if x!=0]) for component in components if 0 in component.toMatch]

def getAllScores(bridges: List[List[List[Component]]]) -> List[int]:
    scores = []
    for group in bridges:
        for bridge in group:
            score = sum([b.score for b in bridge])
            scores.append(score)
    return scores

def getAllLengthScores(bridges: List[List[List[Component]]]) -> List[Dict]:
    scores = []
    for group in bridges:
        for bridge in group:
            score = sum([b.score for b in bridge])
            scores.append({"score": score, "length": len(bridge)})
    return scores

TEST_INPUT = """0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"""

if __name__ == "__main__":
    with open("day24_input.txt") as f:
        raw = f.read().split("\n")
    components = parseAllComponents(raw)

    # components = parseAllComponents(TEST_INPUT.split("\n"))
    maxScore = 0
    allLengths = []
    for component in getStartingComponents(components):
        allBridges = getAllBridges(component, components)
        scores = getAllScores(allBridges)
        scoresLength = getAllLengthScores(allBridges)
        allLengths = allLengths + scoresLength
        if max(scores)>maxScore:
            maxScore = max(scores)
    
    # part 1
    print(maxScore)

    # part 2
    maxLength = max([b["length"] for b in allLengths])
    scoresOfMaxLength = [b["score"] for b in allLengths if b["length"]==maxLength]
    print(max(scoresOfMaxLength))


    
   
    

