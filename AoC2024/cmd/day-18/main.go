package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)

	n := 71
	var grid [][]bool
	for i := 0; i < n; i++ {
		grid = append(grid, make([]bool, n))
	}

	var blocks [][2]int
	for scanner.Scan() {
		s := scanner.Text()
		tab := strings.Split(s, ",")
		x, _ := strconv.Atoi(tab[0])
		y, _ := strconv.Atoi(tab[1])
		blocks = append(blocks, [2]int{x, y})
	}

	maxN := 1024
	for i := range blocks {
		if i >= maxN {
			break
		}
		grid[blocks[i][1]][blocks[i][0]] = true
	}

	for i := 1024; i < 3450; i++ {
		grid[blocks[i][1]][blocks[i][0]] = true
		// Find the shortest path using a BFS.
		total := bfs(grid, n)
		if total == -1 {
			fmt.Println(i, blocks[i])
			break
		}
		fmt.Println(total)
	}
}

func bfs(grid [][]bool, n int) int {
	visited := make(map[[2]int]bool)
	var queue [][3]int
	queue = append(queue, [3]int{0, 0, 0})
	moves := [][2]int{{0, 1}, {-1, 0}, {0, -1}, {1, 0}}

	for {
		if len(queue) == 0 {
			break
		}

		// Pop element in front.
		elem := queue[0]
		queue = queue[1:]

		if visited[[2]int{elem[0], elem[1]}] {
			continue
		}
		visited[[2]int{elem[0], elem[1]}] = true

		if elem[0] == n-1 && elem[1] == n-1 {
			return elem[2]
		}

		// Visit neighbors.
		for _, move := range moves {
			if canMove(grid, elem[0]+move[0], elem[1]+move[1]) {
				queue = append(queue, [3]int{elem[0] + move[0], elem[1] + move[1], elem[2] + 1})
			}
		}
	}

	return -1
}

func canMove(grid [][]bool, i int, j int) bool {
	return i >= 0 && i < len(grid) && j >= 0 && j < len(grid[i]) && !grid[i][j]
}
