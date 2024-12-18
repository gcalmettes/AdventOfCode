class Computer:
    def __init__(self, registers, program):
        self.A = registers.get("A")
        self.B = registers.get("B")
        self.C = registers.get("C")
        self.program = program
        self.pointer = 0
        self.out = []

    def get_literal_operand_value(self, operand):
        return operand

    def get_combo_operand_value(self, operand):
        if operand <=3:
            return operand
        if operand == 4:
            return self.A
        if operand == 5:
            return self.B
        if operand == 6:
            return self.C
        if operand ==7:
            raise("operand 7 is reserved")

    def step(self, opcode, operand):
        if opcode > len(self.program) -1:
            return False
        else:
            if opcode == 0:
                num = self.A
                den = self.get_combo_operand_value(operand)
                res = num // 2**den
                self.A = res
                self.pointer += 2
            if opcode == 1:
                res = self.B ^ self.get_literal_operand_value(operand)
                self.B = res
                self.pointer += 2
            if opcode == 2:
                res = self.get_combo_operand_value(operand) % 8
                self.B = res
                self.pointer += 2
            if opcode == 3:
                if not self.A == 0:
                    self.pointer = self.get_literal_operand_value(operand)
                else:
                    self.pointer += 2
            if opcode == 4:
                res = self.B ^ self.C
                self.B = res
                self.pointer += 2
            if opcode == 5:
                res = self.get_combo_operand_value(operand) % 8
                self.out.append(res)
                self.pointer += 2
            if opcode == 6:
                num = self.A
                den = self.get_combo_operand_value(operand)
                res = num // 2**den
                self.B = res
                self.pointer += 2
            if opcode == 7:
                num = self.A
                den = self.get_combo_operand_value(operand)
                res = num // 2**den
                self.C = res
                self.pointer += 2
            return True

    def run(self):
        while True:
            if self.pointer >= len(self.program)-1:
                break
            opcode, operand = self.program[self.pointer:self.pointer+2]
            if not self.step(opcode, operand):
                break
        return ",".join(str(i) for i in self.out)


REGISTERS = {
  "A": 61657405,
  "B": 0,
  "C": 0,
}
PROGRAM = [2,4,1,2,7,5,4,3,0,3,1,7,5,5,3,0]


cp = Computer(REGISTERS, PROGRAM)
p1 = cp.run()

REGISTERS = {
  "A": 2024,
  "B": 0,
  "C": 0,
}
PROGRAM = [0,3,5,4,3,0]

#117440

p2 = 0
print(f"part1: {p1}")
print(f"part2: {p2}")

cp = Computer(REGISTERS, PROGRAM)
print(cp.run())
