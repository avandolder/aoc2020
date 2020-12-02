import sys

valid = 0
for line in sys.stdin.readlines():
    policy, password = list(map(str.strip, line.split(":")))
    bounds, char = policy.split(" ")
    lo, hi = list(map(int, bounds.split("-")))
    if (password[lo - 1] == char) ^ (password[hi - 1] == char):
        valid += 1
print(valid)
