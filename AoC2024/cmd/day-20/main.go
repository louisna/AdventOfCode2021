package main

import (
	"bufio"
	"fmt"
	"math"
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
	distToEnd := pathFinder(grid, iS, jS, iE, jE)
	fmt.Println(distToEnd[[2]int{iS, jS}])

	total := cheat(grid, distToEnd, 20, 100)
	fmt.Println(total)

}

func inBounds(grid [][]bool, i int, j int) bool {
	return i >= 0 && i < len(grid) && j >= 0 && j < len(grid[i])
}

// Finds the distance from the end to each point in the grid.
func pathFinder(grid [][]bool, iS int, jS int, iE int, jE int) map[[2]int]int {
	moves := [][2]int{{0, 1}, {-1, 0}, {0, -1}, {1, 0}}
	out := make(map[[2]int]int)

	var prev [2]int
	current := [2]int{iE, jE}
	dist := 0

	for {
		out[current] = dist
		dist++

		if current == [2]int{iS, jS} {
			return out
		}

		for _, move := range moves {
			new := [2]int{current[0] + move[0], current[1] + move[1]}
			if inBounds(grid, new[0], new[1]) && grid[new[0]][new[1]] && new != prev && new != current {
				fmt.Println(new, current, prev)
				prev = current
				current = new
				break
			}
		}
	}
}

func manhathanDist(a [2]int, b [2]int) int {
	return int(math.Abs(float64(a[0]-b[0])) + math.Abs(float64(a[1]-b[1])))
}

func cheat(grid [][]bool, distances map[[2]int]int, cheatLen int, mustImprobeBy int) int {
	total := 0

	// Iterate over valid nodes to find a cheat.
	for node := range distances {
		allCandidates := getAllDist(len(grid), node, cheatLen)
		for _, candidate := range allCandidates {
			v, ok := distances[[2]int{candidate[0], candidate[1]}]
			if ok && v+candidate[2] <= distances[node]-mustImprobeBy {
				total++
			}
		}
	}

	return total
}

func getAllDist(n int, from [2]int, dist int) [][3]int {
	var out [][3]int

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			node := [2]int{i, j}
			if manhathanDist(node, from) <= dist {
				nodeDist := [3]int{i, j, manhathanDist(node, from)}
				out = append(out, nodeDist)
			}
		}
	}

	return out
}
