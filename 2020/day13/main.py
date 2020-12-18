
earliest = None
bus_ids = []

with open('input.txt', 'r') as f:
    earliest = int(f.readline())
    bus_ids = [(i, int(bid))
               for i, bid in enumerate(f.readline().split(',')) if bid != 'x']

# print(bus_ids)


def part1():
    min_waiting_time = 9e99
    earliest_bus = -1
    for _, bid in bus_ids:
        waiting_time = (earliest // bid + 1) * bid - earliest
        if waiting_time < min_waiting_time:
            min_waiting_time = waiting_time
            earliest_bus = bid

    print("Result", min_waiting_time * earliest_bus)


mod = 1111111


def part2():
    earliest = 9e999
    counter = 120627261761
    # sorted_bus_ids = sorted(bus_ids, key=lambda pair: pair[1], reverse=True)
    largest_bid_offset, largest_bid = max(bus_ids, key=lambda pair: pair[1])
    found = False
    while not found:
        t = largest_bid * counter - largest_bid_offset
        do = counter % mod == 0
        if do:
            print(counter, t)
        found = True
        for offset, bid in bus_ids:
            if (t + offset) % bid != 0:
                if (do):
                    print('failed on', bid)
                found = False
                break
        earliest = t
        counter += 1

    print(earliest)


# part1()
part2()
