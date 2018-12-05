"""
http://adventofcode.com/2017/day/5
"""

def escapeMaze(numList):
    count = 1

    instructions = numList[:]

    currentPos = 0
    currentJump = instructions[currentPos]
    nextPos = currentPos+currentJump
    while ((currentPos>=0) & (currentPos<len(numList))):
        instructions[currentPos]+=1 #update jump instruction before leaving
        currentPos = nextPos #go to next pos
        if ((currentPos<0) | (currentPos>=len(numList))):
            break
        currentJump = instructions[currentPos]
        nextPos = currentPos+currentJump
        count+=1
    return count


# part 2

def escapeMaze2(numList):
    count = 1

    instructions = numList[:]

    currentPos = 0
    currentJump = instructions[currentPos]
    nextPos = currentPos+currentJump
    while ((currentPos>=0) & (currentPos<len(numList))):
        if currentJump >=3:
            offset = (-1)
        else:
            offset = 1
        instructions[currentPos]+=offset #update jump instruction before leaving
        currentPos = nextPos #go to next pos
        if ((currentPos<0) | (currentPos>=len(numList))):
            break
        currentJump = instructions[currentPos]
        nextPos = currentPos+currentJump
        count+=1
    return count


if __name__ == "__main__":
    with open("day05_input.txt", "r") as f:
        INPUT = [int(line) for line in f]
    # part 1
    print(escapeMaze(INPUT))
    # part 2
    print(escapeMaze2(INPUT))
