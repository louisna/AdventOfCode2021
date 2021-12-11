def main(filename: str, nb_days: int = 256):
    with open(filename) as fd:
        lines = fd.readlines()

    nbrs = [0] * nb_days  # Population born each day
    global_counter = 0
    for number in lines[0].split(","):
        nbrs[int(number)] += 1
        global_counter += 1
    
    for t, nb in enumerate(nbrs):
        global_counter += nb
        # New generation of fish
        if t + 6 + 1 < nb_days:
            nbrs[t + 6 + 1] += nb
    
        if t + 8 + 1 < nb_days:
            nbrs[t + 8 + 1] += nb

    print(global_counter)

    
if __name__ == "__main__":
    main("inputs/input06.txt", 256)