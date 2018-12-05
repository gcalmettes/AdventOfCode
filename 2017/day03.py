"""
http://adventofcode.com/2017/day/3
"""

from collections import defaultdict

"""
Super hacky and ugly code, but does the job ...
"""

INPUT = 265149

def nInRing(idx):
    """
    Get number of values in the Ring # and ring width
    """
    if idx ==0:
        nvals = 1
        width = 1
        return [nvals, width]
    else:
        nvals = (1 + (idx-1) * 2 ) * 4 + 4
        width = (1 + idx * 2 )
        return [nvals, width]

def getRing(n):
    """
    Get the ring # in which the value n lies, as well as
    the ring width and value of bottom right corner or ring
    """
    cond = False
    sum = 0
    i = 0
    while (cond != True):
        nvals, width = nInRing(i)
        sum += nvals
        if n-1<sum:
            cond=True
            return [i, width, nvals, sum-nvals+1]
        i += 1

def getCoords(n):
    idx,width,nvals,firstVal = getRing(n)
    segment = width - 1

    if n == 1:
        coords = (idx, idx)
    elif n == firstVal:
        coords = (idx, idx-1)
    elif n <= firstVal + segment - 1:
        coords = (idx, (idx-1)-(n-firstVal))
    elif n <= firstVal + width - 2 + segment:
        coords = (idx - (n - (firstVal + width - 2)), idx - segment)
    elif n <= firstVal + width - 2 + 2 * segment - 1:
        coords = (-idx, idx - (firstVal + nvals - width - n))
    else:
        coords = (idx - ((firstVal + nvals - 1)-n), idx)

    return coords


assert getCoords(0) == (0, 0)
assert getCoords(1) == (0, 0)
assert getCoords(2) == (1, 0)
assert getCoords(3) == (1, -1)
assert getCoords(4) == (0, -1)
assert getCoords(5) == (-1, -1)
assert getCoords(6) == (-1, 0)
assert getCoords(7) == (-1, 1)
assert getCoords(8) == (0, 1)
assert getCoords(9) == (1, 1)
assert getCoords(10) ==  (2, 1)
assert getCoords(11) ==  (2, 0)
assert getCoords(12) ==  (2, -1)
assert getCoords(13) ==  (2, -2)
assert getCoords(14) ==  (1, -2)
assert getCoords(15) ==  (0, -2)
assert getCoords(16) ==  (-1, -2)
assert getCoords(17) ==  (-2, -2)
assert getCoords(18) ==  (-2, -1)
assert getCoords(19) ==  (-2, 0)
assert getCoords(20) ==  (-2, 1)
assert getCoords(21) ==  (-2, 2)
assert getCoords(22) ==  (-1, 2)
assert getCoords(23) ==  (0, 2)
assert getCoords(24) ==  (1, 2)

def getTaxiDist(n):
    x,y = getCoords(n)
    return abs(x) + abs(y)

# part 2

def getNeighbors(coords):
    x,y = coords
    return [(x+1, y),
            (x+1, y-1),
            (x, y-1),
            (x-1, y-1),
            (x-1, y),
            (x-1, y+1),
            (x, y+1),
            (x+1, y+1)
            ]


def getFirstHigherValue(n):
    coordsDict = defaultdict(int)
    coordsDict[(0, 0)] = 1

    smaller = True
    i = 2
    while smaller:
        coords = getCoords(i)
        value = sum([coordsDict[key] for key in getNeighbors(coords)])
        coordsDict[coords] = value
        i += 1
        if value> n:
            smaller=False
            print(value)


if __name__ == "__main__":
    # part 1
    print(getTaxiDist(INPUT))

    # part 2
    getFirstHigherValue(INPUT)
