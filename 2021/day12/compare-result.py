def process(file):
    return sorted([l.strip() for l in file.readlines()])

with open("output.txt") as outf, open("expected.txt") as expf:
    lout = process(outf)
    lexp = process(expf)

    i = 0
    for e in lexp:
        if e not in lout:
            print(f"({i}) Missing: {e}")
            i += 1
    
    for o in lout:
        if o not in lout:
            print(f"Unexpected: {e}")
