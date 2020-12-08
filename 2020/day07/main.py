
rules = None

target = 'shiny gold'
min_depth = 1

with open("input.txt", 'r') as f:
    rules = f.read().splitlines()


def parse_rule_str(rule):
    container, contents = rule.split(' bags contain ')
    contents_list = []
    for bag in contents.split(', '):
        if 'no other' in bag.strip():
            continue
        bag_str = bag.split(' bag')[0].strip()
        amt = int(bag_str[0])
        contents_list.append({
            'amt': amt,
            'color': bag_str[2:]
        })
    return container, contents_list


all_bags = {c: l for c, l in (parse_rule_str(r) for r in rules)}

found = set()


def dfs(bags, target, depth=0):
    count = 0
    for col, contents in bags.items():
        if col == target and depth > min_depth:
            return 1
        additional = dfs({c['color']: all_bags[c['color']]
                          for c in contents}, target, depth + 1)
        if additional:
            found.add(col)
        count += additional
    return count


# print(dfs(all_bags, target))
# print(len(found))


def d2t(content):
    return content['amt'], content['color']


cache = {}


def dfs2(col):
    count = 1

    contents = [d2t(b) for b in all_bags[col]]

    print('checking for', col)

    if len(contents) == 0:
        return count

    for amt, color in contents:
        print(col, '=> color', color)
        if color not in cache:
            print('not in cache')
            cache[color] = dfs2(color)
        print(color, 'contains', cache[color],
              'bags', 'adding', amt * cache[color])
        count += amt * cache[color]
        print(count)
        print()

    return count


print(dfs2(target))
