package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
	"strings"
)

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)

	var grid [][]int
	var iS, jS, iE, jE int
	j := 0
	for scanner.Scan() {
		s := scanner.Text()
		tab := strings.Split(s, "")

		line := make([]int, len(tab))
		for i := 0; i < len(line); i++ {
			if tab[i] == "#" {
				line[i] = 1
			} else {
				line[i] = 0

				if tab[i] == "S" {
					iS = j
					jS = i
				} else if tab[i] == "E" {
					iE = j
					jE = i
				}
			}
		}
		grid = append(grid, line)
		j++
	}

	// Call dikstra from the start to the end.
	total := dijkstra(grid, iS, jS, iE, jE)

	fmt.Println(total)
}

func dijkstra(grid [][]int, iS int, jS int, iE int, jE int) int {
	moves := [][2]int{{0, 1}, {-1, 0}, {0, -1}, {1, 0}}
	// Heap initialisation.
	h := &IntHeap{}
	heap.Init(h)

	// Add first element, the source.
	heap.Push(h, [7]int{0, iS, jS, 0, iS, jS, 0})

	// Already visited nodes + direction.
	visited := make(map[[3]int]int)

	// To determine where we come from.
	parents := make(map[[3]int][][3]int)

	// Best cost to reach the end.
	bestCost := -1

	for {
		// Pop element from the heap.
		elem := heap.Pop(h).([7]int)
		cost := elem[0]
		i := elem[1]
		j := elem[2]
		moveIdx := elem[3]
		fromI := elem[4]
		fromJ := elem[5]
		fromMoveIdx := elem[6]

		// Avoid going again if we visited this node already.
		v, ok := visited[[3]int{i, j, moveIdx}]
		if ok {
			if v == cost {
				parents[[3]int{i, j, moveIdx}] = append(parents[[3]int{i, j, moveIdx}], [3]int{fromI, fromJ, fromMoveIdx})
			}
			continue
		}
		parents[[3]int{i, j, moveIdx}] = append(parents[[3]int{i, j, moveIdx}], [3]int{fromI, fromJ, fromMoveIdx})
		visited[[3]int{i, j, moveIdx}] = cost

		// Goal objective?
		if i == iE && j == jE {
			if bestCost == -1 {
				bestCost = cost
			} else if bestCost == cost {
				continue // Iterate to get all best paths.
			} else {
				// Higher best cost!
				// Compute the result and return.
				fmt.Println("Compute best score", bestCost, cost)
				return computePathTiles(parents, iE, jE)
			}
		}

		// Move forward if possible.
		nextMove := moves[moveIdx]
		nI := i + nextMove[0]
		nJ := j + nextMove[1]

		if grid[nI][nJ] != 1 {
			heap.Push(h, [7]int{cost + 1, nI, nJ, moveIdx, i, j, moveIdx})
		}

		// Change direction. Disable going backwards.
		nextMove = moves[(moveIdx+1)%4]
		nI = i + nextMove[0]
		nJ = j + nextMove[1]
		if grid[nI][nJ] != 1 {
			heap.Push(h, [7]int{cost + 1000, i, j, (moveIdx + 1) % 4, i, j, moveIdx})
		}

		nextMove = moves[(moveIdx+3)%4]
		nI = i + nextMove[0]
		nJ = j + nextMove[1]
		if grid[nI][nJ] != 1 {
			heap.Push(h, [7]int{cost + 1000, i, j, (moveIdx + 3) % 4, i, j, moveIdx})
		}
	}
}

func computePathTiles(parents map[[3]int][][3]int, iE int, jE int) int {
	end := [3]int{iE, jE, 0}
	end2 := [3]int{iE, jE, 1}

	// Set of all visited nodes.
	visited := make(map[[3]int]bool)
	visitedTiles := make(map[[2]int]bool)

	// Stack of visited.
	var stack [][3]int

	stack = append(stack, end)
	stack = append(stack, end2)

	for {
		if len(stack) == 0 {
			return len(visitedTiles)
		}

		// Pop element from stack.
		elem := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		if visited[elem] {
			continue
		}
		visited[elem] = true
		visitedTiles[[2]int{elem[0], elem[1]}] = true

		stack = append(stack, parents[elem]...)
	}
}

// Définition d'un type pour le Min-Heap
type IntHeap [][7]int

// Implémentation de l'interface heap.Interface
func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i][0] < h[j][0] } // Min-Heap: parent < enfants
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

// Ajout d'un élément dans le heap
func (h *IntHeap) Push(x interface{}) {
	*h = append(*h, x.([7]int))
}

// Suppression du plus petit élément
func (h *IntHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}
