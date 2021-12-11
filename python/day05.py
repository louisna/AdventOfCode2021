import re


def main(filename: str):
    with open(filename, "r") as fd:
        lines = fd.readlines()
    # Contains nbr of overlaping lines
    idx_to_val = dict()

    # Fill in struct with the values
    for line in lines:
        # nbrs conrains the four numbers x1, y1, x2, y2
        nbrs = re.findall(r'\d+', line)
        x1, y1, x2, y2 = int(nbrs[0]), int(nbrs[1]), int(nbrs[2]), int(nbrs[3])
        # Find if vertical, horizontal (or diagonal?)
        is_ver, is_hor = x1 == x2, y1 == y2

        if is_hor:
            for i in range(min(x1, x2), max(x1, x2) + 1):
                idx_to_val[(i, y1)] = idx_to_val.get((i, y1), 0) + 1
        elif is_ver:
            for j in range(min(y1, y2), max(y1, y2) + 1):
                idx_to_val[(x1, j)] = idx_to_val.get((x1, j), 0) + 1
        else:  # Diagonal line
            for shift in range(abs(x1-x2) + 1):
                if x1 > x2: x = x1 - shift
                else: x = x1 + shift

                if y1 > y2: y = y1 - shift
                else: y = y1 + shift
                idx_to_val[(x, y)] = idx_to_val.get((x, y), 0) + 1
    
    # Count number of entries where there is more than one intersection
    print(len(list(filter(lambda x: x > 1, idx_to_val.values()))))


if __name__ == "__main__":
    main("inputs/input05.txt")