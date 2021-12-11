def main(filename: str):
    with open(filename) as fd:
        numbers = [int(i) for i in fd.readlines()[0].split(",")]
    
    best_score = len(numbers) * len(numbers) ** 2
    best_hor = -1

    min_val = min(numbers)
    max_val = max(numbers)

    for i in range(min_val, max_val + 1):
        pot_best_score = compute_diff2(numbers, i)
        if pot_best_score < best_score:
            best_score = pot_best_score
            best_hor = i
    print(best_hor, best_score)


def compute_diff(numbers, i):
    return sum([abs(nbr - i) for nbr in numbers])


def compute_diff2(numbers, i):
    total = 0
    for nbr in numbers:
        y = abs(nbr - i)
        total += y * (y + 1) / 2
    return total


if __name__ == "__main__":
    main("inputs/input07.txt")