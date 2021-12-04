from intcodeComputer import IntcodeComputer


with open('day23_input.txt') as f:
    program = [int(x) for x in f.read().split(',')]

def run_network_part1(program):
    network = {
        i: {
            'cpu': IntcodeComputer(program, inputs=[i]),
            'queue': []
            }
            for i in range(50)
    }
    for cpu in network:
        network[cpu]['gen'] = network[cpu]['cpu'].run()

    i = 0

    while True:
        computer = network[i % len(network)]['cpu']
        gen = network[i % len(network)]['gen']
        queue = network[i % len(network)]['queue']
        if len(queue) > 0:
            for data in queue:
                network[i % len(network)]['cpu'].insert_input(data)
            network[i % len(network)]['queue'] = [] # reset
        else:
            network[i % len(network)]['queue'].insert(0, -1)
            # reset generator
            network[i % len(network)]['gen'] = network[i % len(network)]['cpu'].run()

        try:
            address = next(gen)
            if address:
                if address == 255:
                    x, y = next(gen), next(gen)
                    return y
                    break
                else:
                    x, y = next(gen), next(gen)
                    for data in [x, y]:
                        network[address]['queue'].append(data)    
                    
                    # print(i % len(network), '|', address, x, y)
        except:
            pass
            
        i += 1

def run_network_part2(program):
    network = {
        i: {
            'cpu': IntcodeComputer(program, inputs=[i]),
            'queue': [],
            'idle': False,
            'idle_counter': 0
            }
            for i in range(50)
    }
    for cpu in network:
        network[cpu]['gen'] = network[cpu]['cpu'].run()

    NAT = {
        'previous_packet': [-1, -1],
        'current_packet': []
    }

    sent = [-1, -1]

    i = 0

    while True:
        computer = network[i % len(network)]['cpu']
        gen = network[i % len(network)]['gen']
        queue = network[i % len(network)]['queue']
        if len(queue) > 0:
            for data in queue:
                network[i % len(network)]['cpu'].insert_input(data)
            network[i % len(network)]['queue'] = [] # reset
        else:
            network[i % len(network)]['queue'].insert(0, -1)
            # reset generator
            network[i % len(network)]['gen'] = network[i % len(network)]['cpu'].run()

        try:
            address = next(gen)
            if address:
                network[i % len(network)]['idle'] = False
                network[i % len(network)]['idle_counter'] = 0
                if address == 255:
                    x, y = next(gen), next(gen)
                    NAT['current_packet'] = [x, y]
                    # print(f"NAT received {NAT['current_packet']}")
                else:
                    x, y = next(gen), next(gen)
                    for data in [x, y]:
                        network[address]['queue'].append(data)    
                    
                    # print(i % len(network), '|', address, x, y)
        except:
            network[i % len(network)]['idle_counter'] += 1
            if network[i % len(network)]['idle_counter'] > 50:
                # print(f'------ {i % len(network)} has been declared inactive')
                network[i % len(network)]['idle'] = True

        i += 1

        idle = all(network[j]['idle'] for j in range(len(network)))
        if idle and len(NAT['current_packet']) < 1:
            continue
        elif idle and len(NAT['current_packet']) > 0:
            for data in NAT['current_packet']:
                network[0]['cpu'].insert_input(data)
            # print(f"NAT sent {NAT['current_packet']}")
            if NAT['current_packet'][1] == NAT['previous_packet'][1]:
                # I should break here ... but somehow it doesn't give me the correct result ...
                # print(NAT['current_packet'][1])
                pass
                # break
            else:
                NAT['previous_packet'] = NAT['current_packet'][:]
        else:
            continue
        
        if i > 100000:
            # if I wait for the NAT output values to totally stabilize
            # then this is the right answer ....
            print('part 2:', NAT['current_packet'][1])
            break



part1 = run_network_part1(program)
print(f'part 1: {part1}')

run_network_part2(program)

