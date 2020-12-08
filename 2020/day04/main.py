lines_str = None

with open('input.txt', 'r') as f:
    lines_str = f.read()

passports = [[f.split(':') for f in p.split()]
             for p in lines_str.split('\n\n')]

# print(passports)
[print(len(p), p) for p in passports]

REQUIRED = [
    'byr',
    'iyr',
    'eyr',
    'hgt',
    'hcl',
    'ecl',
    'pid',
]


def do():
    nValid = 0

    for passport in passports:
        valid_fields = 0
        for f, v in passport:
            if f == 'byr':
                if v.isdigit() and 1920 <= int(v) <= 2002:
                    valid_fields += 1
                else:
                    break
            elif f == 'iyr':
                if v.isdigit() and 2010 <= int(v) <= 2020:
                    valid_fields += 1
                else:
                    break
            elif f == 'eyr':
                if v.isdigit() and 2020 <= int(v) <= 2030:
                    valid_fields += 1
                else:
                    break
            elif f == 'hgt':
                val, unit = v[:-2], v[-2:]
                try:
                    val = int(val)
                except:
                    break
                if unit == 'in':
                    if 59 <= val <= 76:
                        valid_fields += 1
                    else:
                        break
                elif unit == 'cm':
                    if 150 <= val <= 193:
                        valid_fields += 1
                    else:
                        break
                else:
                    break
            elif f == 'hcl':
                if not v.startswith('#'):
                    break
                try:
                    int(v[1:], 16)
                    valid_fields += 1
                except:
                    break

            elif f == 'ecl':
                if v in 'amb blu brn gry grn hzl oth'.split():
                    valid_fields += 1
                else:
                    break

            elif f == 'pid':
                if len(v) == 9 and v.isdigit():
                    valid_fields += 1
                else:
                    break

        else:
            if valid_fields != len(REQUIRED):
                continue
            nValid += 1

    print(nValid)


do()
