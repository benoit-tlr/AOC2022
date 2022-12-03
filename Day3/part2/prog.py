from string import ascii_lowercase as alphabet

with open('input') as f:
    lines = f.readlines()
    total_priority = 0

    lines_tuples = [[lines[3*i], lines[3*i+1], lines[3*i+2]] for i in range(len(lines) // 3)]

    for tuple in lines_tuples:
        char = [c for c in tuple[0] if c in tuple[1] and c in tuple[2]][0]
        
        score = 26 if char.isupper() else 0
        score += 1 + alphabet.index(char.lower())

        total_priority += score

    print("Result: " + str(total_priority))
