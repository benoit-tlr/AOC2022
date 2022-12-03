from string import ascii_lowercase as alphabet

with open('input') as f:
    lines = f.readlines()
    total_priority = 0

    for line in lines:
        l = int(len(line) / 2)

        char = [c for c in line[:l] if c in line[l:]][0]
        
        score = 26 if char.isupper() else 0
        score += 1 + alphabet.index(char.lower())

        total_priority += score

    print("Result: " + str(total_priority))
