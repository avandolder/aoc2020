import sys

bd = list(map(str.strip, sys.stdin.readlines()))
n = len(bd)
m = len(bd[0])
prod = 1
steps = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
for (dx, dy) in steps:
    trees = 0
    for (x, y) in zip(range(0, n*dx, dx), range(0, n, dy)):
        if bd[y][x % m] == "#":
            trees += 1
    prod *= trees
print(prod)
