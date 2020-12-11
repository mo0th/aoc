input_seats = None

with open('input.txt', 'r') as f:
    input_seats = [list(r) for r in f.read().split()]


def count_surrouding(seats, row, col, char):
    ops = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]

    nrows = len(seats)
    ncols = len(seats[0])

    count = 0

    for rdelta, cdelta in ops:
        if 0 <= row + rdelta < nrows and 0 <= col + cdelta < ncols:
            if seats[row + rdelta][col + cdelta] == char:
                count += 1

    return count


def tick(seats, n_for_fill=0, n_for_leave=4, count_fn=count_surrouding):
    copy = [r.copy() for r in seats]
    for i, row in enumerate(seats):
        for j, char in enumerate(row):
            if char == '.':
                continue
            count = count_fn(seats, i, j, '#')
            if char == 'L' and count == n_for_fill:
                copy[i][j] = '#'
            elif char == '#' and count >= n_for_leave:
                copy[i][j] = 'L'
    return copy


def count_filled(seats):
    return sum(r.count('#') for r in seats)


def print_seats(seats):
    for row in seats:
        print(''.join(row))


def part1():
    prev = None
    curr = input_seats
    while prev != curr:
        prev, curr = curr, tick(curr)

    print('part 1 =', count_filled(curr))


def count_visibile(seats, row, col, char):
    dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    count = 0

    nrows = len(seats)
    ncols = len(seats[0])

    for x, y in dirs:
        delta = 1
        rdelta = delta * x
        cdelta = delta * y
        while 0 <= (row + rdelta) < nrows and 0 <= (col + cdelta) < ncols:
            if (ch := seats[row+rdelta][col+cdelta]) in ['L', '#']:
                if ch == char:
                    count += 1
                break
            delta += 1
            rdelta = delta * x
            cdelta = delta * y

    return count


def part2():
    prev = None
    curr = input_seats
    n = 0
    while prev != curr:
        prev, curr = curr, tick(curr, n_for_leave=5, count_fn=count_visibile)
        n += 1

    print('part 2 =', count_filled(curr))


# part1()
part2()
