def main(filename):
    with open(filename) as fd:
        data = fd.readlines()
    
    poly = data[0].strip()
    last_letter = poly[-1]

    template = dict()
    for line in data[2:]:
        sp = line.strip().split()
        template[sp[0]] = sp[-1]
    
    poly_dict = dict()
    for i in range(len(poly) - 1):
        first = poly[i]
        second = poly[i+1]
        poly_dict[f"{first}{second}"] = 1
    
    for iter in range(40):
        new_poly = dict()
        for k, v in poly_dict.items():
            middle_val = template[k]
            new_poly[f"{k[0]}{middle_val}"] = v + new_poly.get(f"{k[0]}{middle_val}", 0)
            new_poly[f"{middle_val}{k[1]}"] = v + new_poly.get(f"{middle_val}{k[1]}", 0)
        if iter == 9:
            print(f"Result for iter 9: {count_letters(new_poly, last_letter)}")
        poly_dict = new_poly
    print(f"Result for iter 40: {count_letters(new_poly, last_letter)}")


def count_letters(poly, last_letter):
    totals = dict()
    for k, v in poly.items():
        totals[k[0]] = totals.get(k[0], 0) + v
    totals[last_letter] = totals.get(last_letter, 0) + 1
    values = totals.values()
    return max(values) - min(values)


if __name__ == "__main__":
    main("inputs/input14.txt")