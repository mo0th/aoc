import re

prog = None

with open('input.txt', 'r') as f:
    prog = f.read().splitlines()


def apply_mask(mask, value):
    result = 0
    len_mask = len(mask)
    binary_value = bin(value)[2:].rjust(len_mask, '0')

    for i, mask_bit in enumerate(mask):
        exp = len_mask - i - 1
        value_bit = binary_value[i]
        if mask_bit == '1':
            result += 2 ** exp
        elif mask_bit == 'X':
            add = (2 ** exp) * int(value_bit)
            result += add

    return result


def part1():
    mem = {}
    mask = 'X' * 36

    mem_re = re.compile(r'mem\[(?P<addr>\d+)\] = (?P<value>\d+)')
    mask_re = re.compile(r'mask = (?P<value>[0-9X]+)')

    for op in prog:
        if op.startswith('mask'):
            mask = mask_re.match(op).group('value')
        elif op.startswith('mem'):
            matches = mem_re.match(op).groupdict()
            masked_value = apply_mask(mask, int(matches['value']))
            mem[matches['addr']] = masked_value

    print(sum(mem.values()))


def get_masked_addrs(mask, addr):
    result = []
    len_mask = len(mask)
    binary_value = bin(int(addr))[2:].rjust(len_mask, '0')

    floating_addr = ['0' for _ in range(36)]
    for j, mask_bit in enumerate(mask):
        if mask_bit in ['1', 'X']:
            floating_addr[j] = mask_bit
        elif mask_bit == '0':
            floating_addr[j] = binary_value[j]

    num_floating = floating_addr.count('X')

    for i in range(2 ** num_floating):
        new_addr = floating_addr.copy()
        bits = bin(i)[2:].rjust(num_floating, '0')
        num_bits_used = 0

        for j, bit in enumerate(floating_addr):
            if bit == 'X':
                new_addr[j] = bits[num_bits_used]
                num_bits_used += 1

        result.append(int(''.join(new_addr), 2))

    return result


def part2():
    mem = {}
    mask = 'X' * 36

    mem_re = re.compile(r'mem\[(?P<addr>\d+)\] = (?P<value>\d+)')
    mask_re = re.compile(r'mask = (?P<value>[0-9X]+)')

    for op in prog:
        if op.startswith('mask'):
            mask = mask_re.match(op).group('value')
        elif op.startswith('mem'):
            matches = mem_re.match(op).groupdict()
            masked_addrs = get_masked_addrs(mask, matches['addr'])
            for addr in masked_addrs:
                mem[addr] = matches['value']

    print(sum(int(i) for i in mem.values()))


# part1()
part2()
