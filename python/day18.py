import math


def explode(line, i):
    linevec = line
    a = int(line[i])
    b = int(line[i+2])
    j = i + 2

    # Add value to the left
    idx_left = i - 1
    while idx_left >= 0 and (linevec[idx_left] == "]" or linevec[idx_left] == "[" or linevec[idx_left] == ","):
        idx_left -= 1
    if idx_left < 0: pass
    else: 
        new_val = int(linevec[idx_left]) + a
        linevec[idx_left] = str(new_val)

    # Add value to the right
    idx_right = j + 1
    while idx_right < len(linevec) and (linevec[idx_right] == "]" or linevec[idx_right] == "[" or linevec[idx_right] == ","):
        idx_right += 1
    if idx_right < len(linevec):
        new_val = int(linevec[idx_right]) + b
        linevec[idx_right] = str(new_val)

    # Remove this node
    linevec.pop(i+3)
    linevec.pop(i+2)
    linevec.pop(i+1)
    linevec.pop(i)
    linevec[i-1] = "0"

    return linevec


def split_(line: list, i):
    new_line = line.copy()
    val = int(line[i])
    new_left = math.floor(val / 2)
    new_right = math.ceil(val / 2)
    new_line[i] = "]"
    new_line.insert(i, new_right)
    new_line.insert(i, ",")
    new_line.insert(i, new_left)
    new_line.insert(i, "[")
    return new_line


def print_line(line):
    print("".join([str(i) for i in line]))


def find_explosion_and_split(line):
    depth = 0
    new_line = line.copy()
    for i, char in enumerate(line):
        if char == "[": depth += 1
        elif char == "]": depth -= 1
        elif char == ",": continue
        elif depth > 4:
            new_line = explode(line, i)
            return find_explosion_and_split(new_line)
    for i, char in enumerate(line):
        if char == "[" or char == "]" or char == ",": continue
        if int(char) > 9:
            new_line = split_(line, i)
            return find_explosion_and_split(new_line)
    return new_line


def add_numbers(left, right):
    new_vec = ["["] + left + [","] + right + ["]"]
    return new_vec


def magnitude(line, i):
    for char in line[i:]:
        if char == "[":
            left, idx = magnitude(line, i+1)
            right, idx = magnitude(line, idx + 1)
            return 3 * left + 2 * right, idx + 1
        elif char == "]" or char == ",": continue
        else:
            return int(char), i + 1


def read_input2(filename):
    with open(filename) as fd:
        data = fd.readlines()
    all_data = list()
    for line in data:
        all_data.append(list(line.strip()))
    
    total = find_explosion_and_split(all_data[0])
    for line in all_data[1:]:
        tmp = add_numbers(total, find_explosion_and_split(line))
        total = find_explosion_and_split(tmp)
        print_line(total)
    print_line(total)

    max_total = 0
    for i in range(len(all_data)):
        for j in range(i, len(all_data)):
            total = magnitude(find_explosion_and_split(add_numbers(all_data[i], all_data[j])), 0)
            max_total = max(total[0], max_total)
            total = magnitude(find_explosion_and_split(add_numbers(all_data[j], all_data[i])), 0)
            max_total = max(total[0], max_total)
    print(max_total)


if __name__ == "__main__":
    # read_input("inputs/input18.txt")
    read_input2("inputs/input18.txt")