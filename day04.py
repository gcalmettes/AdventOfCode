"""
http://adventofcode.com/2017/day/4
"""

# part 1
def isValid(pp):
    wordsList = pp.split()
    wordsSet = set(wordsList)
    if len(wordsList)==len(wordsSet):
        valid = 1
    else:
        valid = 0
    return valid

def countValidPP(pplist, validityFunc):
    # allPP = pplist.split("\n")
    count = 0
    # for pp in allPP:
    for pp in pplist:
        count += validityFunc(pp)
    return count

def isValid2(pp):
    wordsList = pp.split()
    wordsLettersList = [''.join(sorted(word)) for word in wordsList]
    wordsLettersSet = set(wordsLettersList)
    if len(wordsLettersList)==len(wordsLettersSet):
        valid = 1
    else:
        valid = 0
    return valid

if __name__ == "__main__":
	with open("day04_input.txt", "r") as f:
		INPUT = [line.strip() for line in f]
	# part 1
	print(countValidPP(INPUT, isValid))
	# part 2
	print(countValidPP(INPUT, isValid2))
