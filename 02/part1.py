import sys

valid = 0
for line in sys.stdin.readlines():
    policy, password = list(map(str.strip, line.split(":")))
    bounds, char = policy.split(" ")
    lo, hi = list(map(int, bounds.split("-")))
    if lo <= password.count(char) <= hi:
        valid += 1
print(valid)
