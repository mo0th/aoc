import math

seats = None

with open('input.txt', 'r') as f:
    seats = f.readlines()


def get_seat_id(seat):
    rmin = 0
    rmax = 127
    cmin = 0
    cmax = 7

    for ch in seat[:7]:
        if ch == 'F':
            rmax = math.floor((rmin + rmax) / 2)
        else:
            rmin = math.ceil((rmin + rmax) / 2)

    for ch in seat[7:]:
        if ch == 'L':
            cmax = math.floor((cmin + cmax) / 2)
        else:
            cmin = math.ceil((cmin + cmax) / 2)

    if rmax != rmin or cmax != cmin:
        print(rmax, rmin, cmax, cmin)

    return rmax * 8 + cmax


possible = set(range(8, 126*8 + 1))

ids = set(get_seat_id(s) for s in seats)
possible.difference_update(ids)

bad = set()

for sid in possible:
    if (sid - 1) not in ids and (sid + 1) not in ids:
        bad.add(sid)

possible.difference_update(bad)


print(possible)
