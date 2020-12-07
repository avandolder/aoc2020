from functools import reduce
import sys

# part 1:
# print(sum(len(set(g.replace("\n", ""))) for g in sys.stdin.read().split("\n\n")))

print(sum(
    len(list(reduce(set.intersection,
        (set(p) for p in filter(bool, g.split("\n")))
    )))
    for g in sys.stdin.read().split("\n\n")
))
