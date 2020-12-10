from collections import Counter
import functools
from pprint import pprint

jolts = None

with open("sample.txt", 'r') as f:
    jolts = sorted(int(n) for n in f.readlines())


# Part 1
# diffs = Counter()
# prev = 0
# for i in range(1, len(jolts)):
#     diff = jolts[i] - jolts[i-1]
#     if diff not in [1, 3]:
#         print(i, jolts[i], jolts[i-1])
#     diffs[diff] += 1
# # Device's built-in adapter
# diffs[3] += 1
# diffs[jolts[0]] += 1
# print(diffs[1] * diffs[3])

STEP_SIZES = (1, 2, 3)


def ways_to_reach2(idx):
    res = [0] * (idx + 2)
    res[0] = 1
    res[1] = 2
    res[2] = 4

    for i in range(3, idx + 1):
        for j in range(1, 4):
            if (jolts[i] - jolts[i - j]) not in STEP_SIZES:
                break
            res[i] += res[i - j]

    return res[idx]


print(ways_to_reach2(len(jolts) - 1))
