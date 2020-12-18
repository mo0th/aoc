import pprint

starting_numbers = []

with open('input.txt', 'r') as f:
    starting_numbers = [int(n) for n in f.read().split(',')]


def part1(turn_target=2020):
    num_to_turns = {}
    prev = None

    for i, n in enumerate(starting_numbers):
        num_to_turns[n] = [i + 1]

    prev = starting_numbers[-1]

    for i in range(len(starting_numbers) + 1, turn_target + 1):
        if prev in num_to_turns and len(num_to_turns[prev]) > 1:
            # Has been spoken before
            spoken = num_to_turns[prev][-1] - num_to_turns[prev][-2]
            if spoken not in num_to_turns:
                num_to_turns[spoken] = []
            num_to_turns[spoken].append(i)
            prev = spoken
        else:
            if 0 not in num_to_turns:
                num_to_turns[0] = []
            num_to_turns[0].append(i)
            prev = 0

    print(prev)


def part2():
    part1(30000000)


# part1()
part2()
