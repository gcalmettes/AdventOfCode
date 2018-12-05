"""
http://adventofcode.com/2017/day/8
"""

test = """b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"""

def getAllRegisters(allLines):
	"""
	Get all unique registers and assign zero as starting value
	"""
	registersList = [line.split()[0] for line in allLines]
	return {register: 0 for register in set(registersList)}

def evaluateCondition(instructionLine, dict):
	"""
	Evaluate the condition for a line of instruction
	"""
	register,sign,num = [el for el in instructionLine.split()][-3:]
	return eval(f'dict["{register}"] {sign} {num}')

def applyInstruction(instructionLine, dict):
	register,operation,num = [el for el in instructionLine.split()][:3]
	if operation == "inc":
		dict[f"{register}"] += int(num)
	elif operation == "dec":
		dict[f"{register}"] -= int(num)
	else:
		raise ValueError
	return None


def completeInstructions(allInstructions, dict):
	maxValue = max(dict[key] for key in dict.keys())
	for instruction in allInstructions:
		if evaluateCondition(instruction, dict):
			applyInstruction(instruction, dict)
			maxStep = max(dict[key] for key in dict.keys())
			if maxStep > maxValue:
				maxValue = maxStep
	return maxValue

def getMax(dict):
	return max(dict[key] for key in dict.keys())


if __name__ == "__main__":
	with open("day08_input.txt", "r") as f:
		INPUT = [line.strip() for line in f]
	registers = getAllRegisters(INPUT)
	maxValue = completeInstructions(INPUT, registers)
	# part 1
	print(getMax(registers))
	# part2
	print(maxValue)





