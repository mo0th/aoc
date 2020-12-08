from collections import deque
codes = None

with open('input.txt', 'r') as f:
    codes = f.read().splitlines()

init_prog = [(op, int(arg)) for op, arg in (c.split() for c in codes)]

q = deque()
execed_progs = []


def run_code(prog):
    acc = 0
    instruction_counter = 0
    executed = set()

    while instruction_counter not in executed and instruction_counter < len(prog):
        if (instruction_counter == 631):
            print(1)
        executed.add(instruction_counter)
        op, arg = prog[instruction_counter]
        if op == 'jmp':
            prog_copy = prog.copy()
            prog_copy[instruction_counter] = ('nop', arg)
            q.append(prog_copy)

            instruction_counter += arg

        else:
            if op == 'acc':
                acc += arg
            if op == 'nop':
                prog_copy = prog.copy()
                prog_copy[instruction_counter] = ('jmp', arg)
                q.append(prog_copy)
                pass

            instruction_counter += 1

    if (instruction_counter >= len(prog)):
        print('fin', acc)

    return acc

# print(codes)
# print(run_code(init_prog))


q.append(init_prog)
while len(q):
    prog = q.popleft()
    if (prog in execed_progs):
        continue
    execed_progs.append(prog)
    run_code(prog)
