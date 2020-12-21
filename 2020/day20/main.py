import re
from pprint import pprint

tiles = None

with open('sample.txt', 'r') as f:
    tile_num_re = re.compile(r'Tile (\d*):')
    tiles = []
    for group in f.read().split('\n\n'):
        tile_num_s, *tile = group.split('\n')
        tiles.append((re.match(tile_num_re, tile_num_s).group(1), tile))


pprint(tiles)


def part1():
    pass
