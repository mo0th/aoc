

numbers = None

with open('input.txt', 'r') as f:
    numbers = [int(n) for n in f.readlines()]


def twosum(numbers, sum):
    seen = set()
    for n in numbers:
        if (sum - n) in seen:
            return True
        seen.add(n)

    return False


# target = None
# for i in range(25, len(numbers)):
#     prev = numbers[i-25:i]
#     if not twosum(prev, numbers[i]):
#         target = numbers[i]

target = 15353384
weak_nums = None

for i in range(len(numbers)):
    for j in range(i + 2, len(numbers)):
        nums = numbers[i:j]
        if sum(nums) == target:
            weak_nums = nums

weakness = min(weak_nums) + max(weak_nums)
print(weakness)
