from dataclasses import dataclass


@dataclass
class Grid:
    val_2_idx: dict()
    row_status: list[int]
    col_status: list[int]
    sum_inactive: int

    def add_value(self, val: int) -> bool:
        i, j = self.val_2_idx.get(val, (-1, -1))
        if val == 25:
            print(i, j)
        if i == -1 or j == -1: return False  # Not in the grid
        
        # Update count of inactive sum because present
        self.sum_inactive -= int(val)

        # Update...
        self.row_status[i] -= 1
        # ... maybe won
        if self.row_status[i] == 0: return True

        # Same
        self.col_status[j] -= 1
        if self.col_status[j] == 0: return True
        # Present but not won
        return False
    
    def set_value(self, val: int, i: int, j: int):
        self.val_2_idx[val] = (i, j)


def main(filename: str):
    with open(filename) as fd:
        lines = fd.read().splitlines()

    # First line is number
    numbers = [int(i) for i in lines[0].split(",")]

    # Second line is always '' and we iterate over all grids
    all_grids = list()
    i = 0
    j_length = 0
    current_grid = dict()
    sum_inactive = 0
    for line in lines[2:]:
        if line == "":
            all_grids.append(Grid(current_grid, [j_length] * i, [i] * j_length, sum_inactive))
            current_grid = dict()
            i = 0
            sum_inactive = 0
            continue

        # Line contains numbers
        for j, val in enumerate(line.split()):
            current_grid[int(val)] = (i, j)
            sum_inactive += int(val)
        
        # Update to go to next row
        i += 1
        j_length = len(line.split())
    # Still need to add the last value
    all_grids.append(Grid(current_grid, [j_length] * i, [i] * j_length, sum_inactive))
    
    # Now iterate over numbers and ask which one is the best
    for number in numbers:
        next_grids = list()
        for grid in all_grids:
            complete = grid.add_value(number)
            if complete:
                print(number * grid.sum_inactive)
            else:
                next_grids.append(grid)
        all_grids = next_grids


if __name__ == "__main__":
    main("inputs/input04.txt")