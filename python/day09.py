def main(filepath: str):
    nbrs = list()
    with open(filepath) as fd:
        for line in fd.readlines():
            nbrs.append([int(i) for i in line.strip()])
    
    total = 0
    m, n = len(nbrs), len(nbrs[0])
    idx_lowest = list()
    for i in range(m):
        for j in range(n):
            nb = nbrs[i][j]
            if i - 1 >= 0 and nbrs[i-1][j] <= nb: continue
            if i + 1 < m and nbrs[i+1][j] <= nb: continue
            if j - 1 >= 0 and nbrs[i][j-1] <= nb: continue
            if j + 1 < n and nbrs[i][j+1] <= nb: continue
            total += nb + 1
            idx_lowest.append((i, j))
    print(total)

    visited = set()

    def find_bassins(i, j):
        print(len(visited))
        from_this_point = 1
        v = nbrs[i][j]
        if (i, j) in visited: return 0
        visited.add((i, j))
        if v == 9: return 0
        if i - 1 >= 0 and nbrs[i-1][j] > v:
            from_this_point += find_bassins(i-1, j)
        if i + 1 < m and nbrs[i+1][j] > v:
            from_this_point += find_bassins(i+1, j)
        if j - 1 >= 0 and nbrs[i][j-1] > v:
            from_this_point += find_bassins(i, j-1)
        if j + 1 < n and nbrs[i][j+1] > v:
            from_this_point += find_bassins(i, j+1)
        return from_this_point
    
    # Part 2
    all_bassins = list()
    for i, j in idx_lowest:
        all_bassins.append(find_bassins(i, j))
    
    all_bassins = sorted(all_bassins)
    print(all_bassins)
    print(all_bassins[-1] * all_bassins[-2] * all_bassins[-3])



if __name__ == "__main__":
    main("inputs/input09.txt")