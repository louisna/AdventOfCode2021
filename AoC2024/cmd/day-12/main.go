package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)

	// Construct the grid.
	var grid [][]rune

	for scanner.Scan() {
		s := scanner.Text()

		line := make([]rune, len(s))

		for i, v := range s {
			line[i] = v
		}

		grid = append(grid, line)
	}

	cost := 0
	costPart2 := 0

	// Initiate the visited state.
	var visited [][]bool
	for i := 0; i < len(grid); i++ {
		visited = append(visited, make([]bool, len(grid[i])))
	}

	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[i]); j++ {
			// Avoid visiting the same region.
			if visited[i][j] {
				continue
			}

			surfPerim := dfs(i, j, grid, visited)

			cost += surfPerim[0] * surfPerim[1]
			costPart2 += surfPerim[0] * surfPerim[2]
		}
	}

	fmt.Println(cost)
	fmt.Println(costPart2)
}

func dfs(i int, j int, grid [][]rune, visited [][]bool) [3]int {
	var stack [][2]int
	stack = append(stack, [2]int{i, j})

	moves := [][2]int{{-1, 0}, {0, -1}, {1, 0}, {0, 1}}

	surface := 0
	perim := 0

	// Part 2.
	fences := make(map[[3]int]bool)
	perim2 := 0

	for {
		if len(stack) == 0 {
			break
		}

		// Stack pop.
		point := stack[len(stack) - 1]
		stack = stack[:len(stack) - 1]

		// Avoid visiting the same node multiple times.
		if visited[point[0]][point[1]] {
			continue
		}
		visited[point[0]][point[1]] = true

		// Update surface because this is a valid node.
		surface++

		// Check neighbours. If it is the same region, we will visit them. Otherwise this is a new fence.
		for moveI, move := range moves {
			newPoint := newPos(point, move)

			if sameRegion(grid, point, newPoint) {
				// Stack push.
				stack = append(stack, newPoint)
			} else {
				perim++

				// Part 2.
				key := [3]int{point[0], point[1], moveI}

				// This 
				if fences[key] {
					continue
				}

				perim2++

				visitDirection(grid, point, moveI, fences)
			}
		}
	}

	return [3]int{surface, perim, perim2}
}

func inBounds(grid [][]rune, pos [2]int) bool {
	return pos[0] >= 0 && pos[0] < len(grid) && pos[1] >= 0 && pos[1] < len(grid[pos[0]])
}

func sameRegion(grid [][]rune, from [2]int, to [2]int) bool {
	return inBounds(grid, to) && grid[from[0]][from[1]] == grid[to[0]][to[1]]
}

func newPos(from [2]int, to [2]int) [2]int {
	return [2]int{from[0] + to[0], from[1] + to[1]}
}

func visitDirection(grid [][]rune, pos [2]int, direction int, fences map[[3]int]bool) {
	moves := [][2]int{{-1, 0}, {0, -1}, {1, 0}, {0, 1}}

	// Check if we are still on the edge for that direction.
	dirPoint := newPos(pos, moves[direction])
	if sameRegion(grid, pos, dirPoint) {
		return
	}

	fences[[3]int{pos[0], pos[1], direction}] = true

	to1 := newPos(pos, moves[(direction + 1) % 4])
	to1Key := [3]int{to1[0], to1[1], direction}
	if sameRegion(grid, pos, to1) && !fences[to1Key] {
		visitDirection(grid, to1, direction, fences)
	}

	to2 := newPos(pos, moves[(direction + 3) % 4])
	to2Key := [3]int{to2[0], to2[1], direction}
	if sameRegion(grid, pos, to2) && !fences[to2Key] {
		visitDirection(grid, to2, direction, fences)
	}
}