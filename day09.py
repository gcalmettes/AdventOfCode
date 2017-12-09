"""
http://adventofcode.com/2017/day/9
"""

# def countValid(str):
#     count = 0
#     for i in str:
#         if i == "{":
#             count += 1
#         elif i == "}":
#             count -= 1
#         if count < 0:
#             return False
#     return count == 0
#
# def evaluate(str):
#   stack = []
#   pushChars, popChars = "<({[", ">)}]"
#   for c in str :
#     if c in pushChars :
#       stack.append(c)
#     elif c in popChars :
#       if not len(stack) :
#         return False
#       else :
#         stackTop = stack.pop()
#         balancingBracket = pushChars[popChars.index(c)]
#         if stackTop != balancingBracket :
#           return False
#     else :
#       return False
#   return not len(stack)
#
# def find_parens(s):
#     toret = {}
#     pstack = []
#
#     for i, c in enumerate(s):
#         if c == '{':
#             pstack.append(i)
#         elif c == '}':
#             if len(pstack) == 0:
#                 raise IndexError("No matching closing parens at: " + str(i))
#             toret[pstack.pop()] = i
#
#     if len(pstack) > 0:
#         raise IndexError("No matching opening parens at: " + str(pstack.pop()))
#
#     return toret

def cleanStream(stream):
	streamToClean = stream[:]
	isCleaned = False
	while not isCleaned:
		idx = [i for i,item in enumerate(streamToClean) if item=="!"]
		if len(idx)<1:
			isCleaned = True
			break
		streamToClean = streamToClean[:idx[0]]+streamToClean[idx[0]+2:]
	return streamToClean

def cleanGarbage(stream):
	streamToClean = cleanStream(stream)
	garbage = ""
	hasGarbage = True
	while hasGarbage:
		idxOpen = [i for i,item in enumerate(streamToClean) if item=="<"]
		idxClose = [i for i,item in enumerate(streamToClean) if item==">"]
		if len(idxOpen)<1 and len(idxClose)<1:
			hasGarbage = False
			break
		elif len(idxOpen)<1 or len(idxClose)<1:
			print("Mismatch in garbage parentheses")
			break
		garbage = garbage + streamToClean[idxOpen[0]+1:idxClose[0]]
		streamToClean = streamToClean[:idxOpen[0]]+streamToClean[idxClose[0]+1:]
	return streamToClean, garbage

assert cleanGarbage("{<<<<>}") == ("{}","<<<")
assert cleanGarbage("{<{!>}>}") == ("{}","{}")
assert cleanGarbage("{<!!>}") == ("{}","")
assert cleanGarbage("{<random characters>}") == ("{}","random characters")
assert cleanGarbage("{<>}") == ("{}","")
assert cleanGarbage("{<!!!>>}") == ("{}","")
assert cleanGarbage('{<{o"i!a,<{i<a>}') == ("{}",'{o"i,<{i<a')

def calculateScore(stream):
	streamToScore,garbage = cleanGarbage(stream)
	score = 0
	currentPoint = 0
	for bracket in streamToScore:
		if bracket == "{":
			currentPoint += 1
			score += currentPoint
		elif bracket =="}":
			currentPoint -= 1
	if currentPoint != 0:
		print("Mismatch in parentheses")
	return score

assert calculateScore(cleanGarbage('{}')[0]) == 1
assert calculateScore(cleanGarbage('{{{}}}')[0]) == 6
assert calculateScore(cleanGarbage('{{},{}}')[0]) == 5
assert calculateScore(cleanGarbage('{{{},{},{{}}}}')[0]) == 16
assert calculateScore(cleanGarbage('{<a>,<a>,<a>,<a>}')[0]) == 1
assert calculateScore(cleanGarbage('{{<ab>},{<ab>},{<ab>},{<ab>}}')[0]) == 9
assert calculateScore(cleanGarbage('{{<!!>},{<!!>},{<!!>},{<!!>}}')[0]) == 9
assert calculateScore(cleanGarbage('{{<a!>},{<a!>},{<a!>},{<ab>}}')[0]) == 3


if __name__ == "__main__":
	with open("day09_input.txt") as f:
		# part 1
		cleanedStream,garbage = cleanGarbage(f.read())
		print(calculateScore(cleanedStream))
		# part 2
		print(len(garbage))
