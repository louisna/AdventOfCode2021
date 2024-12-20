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

	var grid [][]int
	var starts [][2]int

	i := 0
	for scanner.Scan() {
		s := scanner.Text()
		
		tab := strings.Split(s, "")

		// Append new tab line.
		grid = append(grid, make([]int, len(tab)))

		for j, valString := range tab {
			val, e := strconv.Atoi(valString)
			if e != nil {
				continue
			}
			grid[i][j] = val

			if val == 0 {
				starts = append(starts, [2]int{i, j})
			}
		}

		i++
	}

	nbTotal := 0
	for _, start := range starts {
		nbTotal += dfs(grid, start)
	}

	fmt.Println(nbTotal)
}

func dfs(grid [][]int, start [2]int) int {
	var stack [][2]int
	stack = append(stack, start)

	moves := [][2]int{{-1, 0}, {0, -1}, {1, 0}, {0, 1}}

	visited := make(map[[2]int]bool)

	nbTotal := 0

	for {
		if len(stack) == 0 {
			break
		}

		// Update stack by removing last element.
		point := stack[len(stack) - 1]
		stack = stack[:len(stack) - 1]

		if visited[point] {
			continue
		}

		// visited[point] = true

		i := point[0]
		j := point[1]

		if grid[i][j] == 9 {
			nbTotal++
			continue
		}

		for _, move := range moves {
			newI := i + move[0]
			newJ := j + move[1]
			
			if inBounds(grid, newI, newJ) && grid[i][j] + 1 == grid[newI][newJ] {
				stack = append(stack, [2]int{newI, newJ})
			}
		}
	}

	return nbTotal
}

func inBounds(grid [][]int, i int, j int) bool {
	return i >= 0 && i < len(grid) && j >= 0 && j < len(grid[i])
}
