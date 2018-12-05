"""
http://adventofcode.com/2017/day/15
"""

from typing import Tuple

def calculateNext(previous: int, factor: int):
    return (previous*factor) % 2147483647

def convertPairToBinary(valA: int, valB: int) -> Tuple[str, str]:
    return (f"{valA:b}".zfill(32), f"{valB:b}".zfill(32))

def isMatch(binA: str, binB: str) -> bool:
    return binA[-16:]==binB[-16:]

def countMatch1(nSample: int, startA: int, startB: int, factorA: int = 16807, factorB: int = 48271) -> int:
    prevA = startA
    prevB = startB

    nMatch = 0
    for i in range(nSample):
        prevA, nextA = prevA, calculateNext(prevA, factorA)
        prevB, nextB = prevB, calculateNext(prevB, factorB)

        binA,binB = convertPairToBinary(nextA, nextB)
        if isMatch(binA, binB):
            nMatch += 1
        prevA, prevB = nextA, nextB

    return nMatch

def countMatch2(nSample: int, startA: int, startB: int, factorA: int = 16807, factorB: int = 48271) -> int:
    prevA = startA
    prevB = startB

    nMatch = 0
    for i in range(nSample):
        gotA = False
        gotB = False

        while not gotA:
            prevA, nextA = prevA, calculateNext(prevA, factorA)
            if nextA % 4 == 0:
                gotA = True
                # print("got A:", nextA)
            else:
                prevA = nextA
        
        while not gotB:
            prevB, nextB = prevB, calculateNext(prevB, factorB)
            if nextB % 8 == 0:
                gotB = True
                # print("got B:", nextB)
            else:
                prevB = nextB

        binA,binB = convertPairToBinary(nextA, nextB)
        if isMatch(binA, binB):
            nMatch += 1
        prevA, prevB = nextA, nextB

    return nMatch


if __name__ == '__main__':

    startA = 591
    startB = 393

    # part 1
    print(countMatch1(40000000, startA, startB))

    # part 2
    print(countMatch2(5000000, startA, startB))

