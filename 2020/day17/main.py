
ACTIVE = '#'
INACTIVE = '.'

initial_state = None

with open('sample.txt', 'r') as f:
    initial_state = f.read().splitlines()
