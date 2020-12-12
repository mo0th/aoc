moves = None

with open('input.txt', 'r') as f:
    moves = [(l[0], int(l[1:])) for l in f.read().split()]

dirs = ['N', 'E', 'S', 'W']


def part1():
    facing = 'E'
    ns_amt = 0
    ew_amt = 0

    def move_boat(direction, amt):
        nonlocal ns_amt, ew_amt
        if direction == 'N':
            ns_amt += amt
        elif direction == 'S':
            ns_amt -= amt
        elif direction == 'E':
            ew_amt += amt
        elif direction == 'W':
            ew_amt -= amt

    for move, amount in moves:
        if move in dirs:
            move_boat(move, amount)
        elif move == 'F':
            move_boat(facing, amount)
        else:
            curr_idx = dirs.index(facing)
            rotations = amount // 90
            if move == 'R':
                facing = dirs[(curr_idx + rotations) % len(dirs)]
            elif move == 'L':
                facing = dirs[(curr_idx - rotations + len(dirs)) % len(dirs)]

    distance = abs(ns_amt) + abs(ew_amt)
    print('manhattan distance', distance)


def part2():
    ns_amt = 0
    ew_amt = 0
    waypoint = [1, 10]

    def move_waypoint(direction, amt):
        nonlocal waypoint
        if direction == 'N':
            waypoint[0] += amt
        elif direction == 'S':
            waypoint[0] -= amt
        elif direction == 'E':
            waypoint[1] += amt
        elif direction == 'W':
            waypoint[1] -= amt

    def move_boat(times):
        nonlocal ns_amt, ew_amt
        ns, ew = waypoint
        for _ in range(times):
            ns_amt += ns
            ew_amt += ew
    for move, amount in moves:
        if move in dirs:
            move_waypoint(move, amount)
        elif move == 'F':
            move_boat(amount)
        else:
            rotations = amount // 90
            for _ in range(rotations):
                ns, ew = waypoint
                if move == 'R':
                    # [4, 10] => [-10, 4]
                    waypoint = [-ew, ns]
                elif move == 'L':
                    # [4, 10] => [10, -4]
                    waypoint = [ew, -ns]
    distance = abs(ns_amt) + abs(ew_amt)
    print('manhattan distance', distance)


# part1()
part2()
