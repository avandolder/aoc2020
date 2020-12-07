import math
import sys

ids = []
for p in sys.stdin.readlines():
    i, j = 0, 127
    for c in p[:7]:
        if c == 'F':
            j = (i + j) // 2
        elif c == 'B':
            i = math.ceil((i + j) / 2)
    m, n = 0, 7
    for c in p[7:10]:
        if c == 'L':
            n = (m + n) // 2
        elif c == 'R':
            m = math.ceil((m + n) / 2)
    ids.append(i * 8 + m)

ids.sort()
for (id1, id2) in zip(ids, ids[1:]):
    if id1 + 1 != id2:
        print(id1 + 1)
        break
