import regex
import sys

def height(h: str) -> bool:
    return ((h.endswith("cm") and 150 <= int(h[:-2]) <= 193) or
            (h.endswith("in") and 59 <= int(h[:-2]) <= 76))

def hcl(s: str) -> bool:
    return regex.fullmatch("#[0-9a-f]{6}", s) is not None

def ecl(s: str) -> bool:
    return s in {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'}

def pid(s: str) -> bool:
    return regex.fullmatch("[0-9]{9}", s) is not None

req = {"byr","iyr","eyr","hgt","hcl","ecl","pid"}

valid = 0
for passport in sys.stdin.read().split("\n\n"):
    fields = passport.replace("\n", " ").split(" ")
    fields = {f[:3]: f[4:] for f in fields}
    #print(fields)
    if (req.issubset(fields.keys()) and
            1920 <= int(fields['byr']) <= 2002 and len(fields['byr']) == 4 and
            2010 <= int(fields['iyr']) <= 2020 and len(fields['iyr']) == 4 and
            2020 <= int(fields['eyr']) <= 2030 and len(fields['eyr']) == 4 and
            height(fields['hgt']) and
            hcl(fields['hcl']) and
            ecl(fields['ecl']) and
            pid(fields['pid'])):
        valid += 1

print(valid)
