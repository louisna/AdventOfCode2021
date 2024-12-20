package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)

	var grid [][]bool

	var iS, jS, iE, jE int

	i := 0
	for scanner.Scan() {
		s := scanner.Text()
		tab := strings.Split(s, "")
		var line []bool
		for j, vs := range tab {
			if vs == "#" {
				line = append(line, false)
			} else {
				line = append(line, true)
				if vs == "S" {
					iS = i
					jS = j
				} else if vs == "E" {
					iE = i
					jE = j
				}
			}
		}
		i++
		grid = append(grid, line)
	}

	// First BFS to compute the baseline.
	baseline := bfs(grid, iS, jS, iE, jE, false, 0)

	fmt.Println("Baseline", baseline)

	// Then the number of walls.
	total := bfs(grid, iS, jS, iE, jE, true, baseline)
	fmt.Println(total)

}

func bfs(grid [][]bool, iS int, jS int, iE int, jE int, goThroughWalls bool, baseline int) int {
	visited := make(map[[6]int]bool)
	moves := [][2]int{{0, 1}, {-1, 0}, {0, -1}, {1, 0}}

	// I, J, Cost, whether we can go through walls.
	// Last value: 0 = cannot go through walls, 1 = currently going through walls, 2 = can go through walls.
	var queue [][8]int

	if goThroughWalls {
		queue = append(queue, [8]int{iS, jS, 0, 2, -1, -1, -1, -1})
	} else {
		queue = append(queue, [8]int{iS, jS, 0, 0, -1, -1, -1, -1})
	}

	value := 0

	for {
		if len(queue) == 0 {
			break
		}

		// Pop elem.
		elem := queue[0]
		queue = queue[1:]

		// Avoid visiting twice the same node.
		if visited[[6]int{elem[0], elem[1], elem[4], elem[5], elem[6], elem[7]}] {
			continue
		}
		visited[[6]int{elem[0], elem[1], elem[4], elem[5], elem[6], elem[7]}] = true

		// Goal.
		if elem[0] == iE && elem[1] == jE {
			if goThroughWalls {
				if elem[2] > baseline - 100 {
					return value
				} else {
					fmt.Println("We go throgh walls", elem[4], elem[5], elem[6], elem[7], "and save", baseline - elem[2])
					value += 1
				}
				continue
			} else {
				return elem[2]
			}
		}

		// Must update whether we can go through wall.
		// If the value is 1, it means that we can only move to a wall THIS turn.
		decrementWall := 0
		if elem[3] == 1 {
			decrementWall++
		}

		// Move.
		for _, move := range moves {
			nextI := elem[0] + move[0]
			nextJ := elem[1] + move[1]
			
			if inBounds(grid, nextI, nextJ) {
				// We can go freely.
				if grid[nextI][nextJ] {
					// Add in queue.
					queue = append(queue, [8]int{nextI, nextJ, elem[2] + 1, elem[3] - decrementWall, elem[4], elem[5], elem[6], elem[7]})
				} else if elem[3] > 0 {
					nextI2 := elem[0] + move[0] * 2
					nextJ2 := elem[1] + move[1] * 2
					if inBounds(grid, nextI2, nextJ2) && grid[nextI2][nextJ2] {
						queue = append(queue, [8]int{nextI2, nextJ2, elem[2] + 2, 0, elem[0], elem[1], nextI2, nextJ2})
					}
				}
			}
		}
	}

	return -1
}

func inBounds(grid [][]bool, i int, j int) bool {
	return i >= 0 && i < len(grid) && j >= 0 && j < len(grid[i])
}
