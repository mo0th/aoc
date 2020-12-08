groups = None

with open('input.txt', 'r') as f:
    groups = f.read()

groups = groups.split('\n\n')

sets = []

for group in groups:
    people = group.split()
    sets.append(set(people[0]))
    for person in people[1:]:
        sets[-1].intersection_update(person)

print(sets)


total = sum(len(s) for s in sets)

print(total)
