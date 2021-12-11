def main(filepath: str):
    with open(filepath) as fd:
        data = fd.readlines()
    part_2(data)


def part_1(data: list[str]):
    # Get output
    count = 0
    for line in data:
        output = line.split(" | ")[-1]

        # Get individually each digit
        digits = output.split()

        lengths_unique = [2, 3, 4, 7]
        count += len([1 for i in digits if len(i) in lengths_unique])
    print(count)


all_digits = [
    "abcefg",
    "cf",
    "acdeg",
    "acdfg",
    "bcdf",
    "abdfg",
    "abdefg",
    "acf",
    "abcdefg",
    "abcdfg"
]


def part_2(data: list[str]):
    total_sum = 0
    for line in data:
        input = line.split(" | ")[0]
        digits = input.split()

        # Map each segment with a counter
        digit_one = ""
        digit_four = ""
        segcount = dict()
        for digit in digits:
            if len(digit) == 2: digit_one = digit
            if len(digit) == 4: digit_four = digit
            for letter in digit:
                segcount[letter] = segcount.get(letter, 0) + 1
        # print(segcount)

        letter_to_seg = dict()

        # Already discovered letter
        discovered = set()

        # We know that "1" is composed of two segments, se we can identify those
        # The upper segment is present 8 times and the lower 9 times
        if segcount[digit_one[0]] == 9:
            letter_to_seg[digit_one[0]] = "f"
            letter_to_seg[digit_one[1]] = "c"
        else:
            letter_to_seg[digit_one[0]] = "c"
            letter_to_seg[digit_one[1]] = "f"
        discovered.add(digit_one[0])
        discovered.add(digit_one[1])
        
        # We can also identify two other segments
        for key, value in segcount.items():
            if value == 4:
                letter_to_seg[key] = "e"
                discovered.add(key)
            if value == 6:
                letter_to_seg[key] = "b"
                discovered.add(key)
        
        # Use the unique "4" to determine other segments
        for letter in digit_four:
            if segcount[letter] == 7:
                letter_to_seg[letter] = "d"
                discovered.add(letter)
                break
                
        # It only remains unique count values
        for key, value in segcount.items():
            if key in discovered: continue
            if value == 8:
                letter_to_seg[key] = "a"
                discovered.add(key)
            elif value == 7:
                letter_to_seg[key] = "g"
                discovered.add(key)
        
        # We know have all letters mapped
        # print(letter_to_seg)
        # Can find the output
        output = line.split(" | ")[1]
        this_result = ""
        for target in output.split():
            # Try every digit lol
            for val, candidate in enumerate(all_digits):
                if len(candidate) != len(target): continue  # Not this one
                is_same = True
                for d in target:
                    if letter_to_seg[d] not in candidate:
                        is_same = False
                        break
                if is_same:
                    this_result += str(val)
                    break
        total_sum += int(this_result)
    print(total_sum)


if __name__ == "__main__":
    main("inputs/input08.txt")