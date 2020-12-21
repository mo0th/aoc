from pprint import pprint
import functools

rules = None
messages = None

with open('sample.txt', 'r') as f:
    rules_s, messages_s = f.read().split('\n\n')

    rules = {}
    for num, rule_s in (rule.split(': ') for rule in rules_s.splitlines()):
        rules[num] = [n.split() for n in rule_s.split(' | ')]
    messages = messages_s.splitlines()


@functools.lru_cache
def matches_rule(rule_n, message, start_idx=0):
    subrule_options = rules[rule_n]

    valid = True
    print(1, repr(rule_n), message, start_idx)
    for subrule in subrule_options:
        i = start_idx
        print(2, subrule)
        if len(message) <= i:
            print(3)
            valid = False
            continue

        if len(subrule) == 1:
            ch = subrule[0][1]
            print(4, repr(ch), repr(message[i]))
            if ch == message[i]:
                print(5)
                valid = True
                i += 1
                continue
            else:
                print(6)
                valid = False
                continue

        for n in subrule:
            print(7)
            if not matches_rule(n, message, i):
                print(8)
                valid = False
                break
            else:
                print(9)
                i += 1
                valid = True

        if valid:
            break

    print(f'returning {valid} {start_idx=} {subrule_options}')
    return valid


def part1():
    # pprint(gen_options())
    print('valid', len(
        [1 for message in messages[:1] if matches_rule('0', message)]))


def part2():
    pass


part1()
# part2()
