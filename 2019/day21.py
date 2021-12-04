from intcode_fogleman import run as runIntcode

def run(script):
    return list(runIntcode(program, script))[-1]

# program = list(fileinput.input())[0]
with open('day21_input.txt') as f:
    program = [int(x) for x in f.read().split(',')]

script = '''
OR C J
AND A J
AND B J
NOT J J
AND D J
WALK
'''.lstrip()

print(run(script))

script = '''
NOT H J
OR C J
AND A J
AND B J
NOT J J
AND D J
RUN
'''.lstrip()

print(run(script))