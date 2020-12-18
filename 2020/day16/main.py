import re

rules = None
my_ticket = None
other_tickets = None

with open('input.txt', 'r') as f:
    rules_s, my_s, other_s = f.read().split('\n\n')

    rule_re = re.compile(
        r'(?P<name>[a-z ]*): (?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)')

    rules = []
    for line in rules_s.split('\n'):
        matches = rule_re.match(line).groupdict()
        rules.append((
            matches['name'],
            (int(matches['min1']), int(matches['max1'])),
            (int(matches['min2']), int(matches['max2']))))

    my_ticket = tuple(int(n) for n in my_s.split('\n')[1].split(','))

    other_tickets = tuple(tuple(int(n) for n in t.split(','))
                          for t in other_s.split('\n')[1:])


def check_rules(rules, num):
    for name, *ranges in rules:
        for min_num, max_num in ranges:
            if min_num <= num <= max_num:
                return True
    return False


def part1():
    invalid_nums = []

    for ticket in other_tickets:
        for num in ticket:
            if not check_rules(rules, num):
                invalid_nums.append(num)

    print(sum(invalid_nums))


def part2():
    valid_tickets = []
    for ticket in other_tickets:
        for num in ticket:
            if not check_rules(rules, num):
                break
        else:
            valid_tickets.append(ticket)

    possible_positions = {name: set(range(len(my_ticket)))
                          for name, *_ in rules}

    for ticket in valid_tickets:
        for name, *ranges in rules:
            for i, num in enumerate(ticket):
                for min_n, max_n in ranges:
                    if min_n <= num <= max_n:
                        break
                else:
                    possible_positions[name].remove(i)

        if all(len(s) == 1 for s in possible_positions.values()):
            break

    final_order = [None for _ in my_ticket]

    while any(n is None for n in final_order):
        added = set()
        for name, positions in possible_positions.items():
            if len(positions) == 1:
                idx = positions.pop()
                final_order[idx] = name
                added.add(idx)

        for n in added:
            for positions in possible_positions.values():
                positions.discard(n)
        pass

    answer = 1
    for field, value in zip(final_order, my_ticket):
        if field.startswith('departure'):
            answer *= value

    print(answer)


# part1()
part2()
