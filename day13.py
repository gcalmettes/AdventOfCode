"""
http://adventofcode.com/2017/day/13
"""

from typing import NamedTuple, List

class Scanner(NamedTuple):
    depth: int = 0
    maxRange: int = 0
    

def willBeOnTop(scanner: Scanner, delay: int) -> bool:
    """
    Scanner will be on top when we arrive at its depth in the firewall
    if the time it takes to reach the  scanner (equal to scanner depth)
    is a multiple of two times the range of the
    scanner (scanner did an out an back)

    If there is a delay, the time it takes to reach the scanner depth
    is (scanner.depth+time)
    """
    return (scanner.depth+delay) % ((scanner.maxRange-1)*2) == 0


def getFirewall(firewall: str) -> List[Scanner]:
    layers = firewall.split("\n")
    scanners = [list(map(int, layer.split(": "))) for layer in layers]
    return [Scanner(scanner[0], scanner[1]) for scanner in scanners]


def getSeverity(scanner: Scanner) -> int:
    return scanner.depth * scanner.maxRange


def getPacketDamages(firewallInput: str, delay: int) -> int:
    firewall = getFirewall(firewallInput)
    caughtAt = [willBeOnTop(scanner, delay) for scanner in firewall]
    severity = sum([getSeverity(scanner) for i,scanner in enumerate(firewall) if caughtAt[i]])
    return severity
    

def isPacketCaught (firewall: List[Scanner], delay: int) -> bool:
    caught = False
    for scanner in firewall:
        if willBeOnTop(scanner, delay):
            caught = True
            break
    return caught



TEST_FIREWALL = """0: 3
1: 2
4: 4
6: 4"""


assert getPacketDamages(TEST_FIREWALL, 0) == 24

if __name__ == '__main__':
    
    with open("day13_input.txt") as f:
        firewallInput = f.read()
        # firewallInput = TEST_FIREWALL

        # part 1
        print(getPacketDamages(firewallInput, 0))
        
        # part 2
        delay = 0
        firewall = getFirewall(firewallInput)

        while isPacketCaught(firewall, delay):
            delay += 1
        print(delay)
