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

	// Create and read the map.
	var m [][]bool

	// Starting position.
	var si, sj int

	i := 0
	for scanner.Scan() {
		s := scanner.Text()

		tab := strings.Split(s, "")
		m = append(m, make([]bool, len(tab)))
		for j, v := range tab {
			if v == "#" {
				m[i][j] = true
			} else if v == "^" {
				si = i
				sj = j
				m[i][j] = false
			} else {
				m[i][j] = false
			}
		}
		i++
	}

	res := visit_once(m, si, sj)
	fmt.Println(res)
}

func visit_once(maze [][]bool, si int, sj int) []int {
	nbVisited := 1

	direction := 0
	nextMove := []struct{ i, j int }{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	visited := make([][]bool, len(maze))
	for i := 0; i < len(maze); i++ {
		visited[i] = make([]bool, len(maze[i]))
	}
	visited[si][sj] = true

	// Visit.
	i, j := si, sj

	for {
		move := nextMove[direction]
		ni := i + move.i
		nj := j + move.j
		if ni < 0 || ni >= len(maze) || nj < 0 || nj >= len(maze[ni]) {
			break
		}
		if maze[ni][nj] {
			direction = (direction + 1) % 4
			continue
		}

		if !visited[ni][nj] {
			nbVisited++
			visited[ni][nj] = true
		}

		i, j = ni, nj
	}

	// Task 2: brute force.
	nbPossible := 0

	for i := 0; i < len(maze); i++ {
		for j := 0; j < len(maze[i]); j++ {
			if maze[i][j] {
				continue
			}

			if i == si && j == sj {
				continue
			}

			newMaze := mazeCopy(maze)
			newMaze[i][j] = true

			if findCycle(newMaze, si, sj, 0) {
				nbPossible++
			}
		}
	}

	return []int{nbVisited, nbPossible}
}

// This function allows to find a cycle in the path.
// A cycle occurs whenever the guard reaches the same position in the same direction as before.
func findCycle(maze [][]bool, si int, sj int, sdir int) bool {
	direction := sdir
	nextMove := []struct{ i, j int }{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	i, j := si, sj

	// Set of visited * direction.
	visited := make(map[[3]int]bool)

	for {
		move := nextMove[direction]
		ni := i + move.i
		nj := j + move.j

		if ni < 0 || ni >= len(maze) || nj < 0 || nj >= len(maze[ni]) {
			return false
		}

		if maze[ni][nj] {
			direction = (direction + 1) % 4
			continue
		}

		if visited[[3]int{ni, nj, direction}] {
			return true
		}
		visited[[3]int{ni, nj, direction}] = true

		i, j = ni, nj
	}
}

func mazeCopy(maze [][]bool) [][]bool {
	other := make([][]bool, len(maze))
	for i := range other {
		other[i] = make([]bool, len(maze[i]))
		copy(other[i], maze[i])
	}
	return other
}
