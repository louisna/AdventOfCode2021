package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Action struct {
	A      string
	B      string
	Action string
}

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)

	graph := make(map[string]Action)
	values := make(map[string]int)

	readInitial := true
	for scanner.Scan() {
		s := scanner.Text()
		if len(s) < 5 {
			readInitial = false
			continue
		}

		if readInitial {
			tab := strings.Split(s, ": ")
			v, _ := strconv.Atoi(tab[1])
			values[tab[0]] = v
		} else {
			tab := strings.Split(s, " ")
			action := Action{
				A:      tab[0],
				B:      tab[2],
				Action: tab[1],
			}
			graph[tab[4]] = action
		}
	}

	for node := range graph {
		if node[0] == 'z' {
			findValueOfZ(graph, node, values, 0)
		}
	}

	// Compute the outgoing result.
	output := computeVal(values, "z")
	fmt.Println(output)

	// Try with some values.
	for i := 1; i < 45; i++ {
		x := 1 << i
		y := 0
		initValues := valToBits(x, y)
		for node := range graph {
			if node[0] == 'z' {
				findValueOfZ(graph, node, initValues, 0)
			}
		}

		output = computeVal(initValues, "z")
		if output != x+y {
			fmt.Printf("X=%d and Y=%d => Z=%d vs %d (%s vs %s)\n", x, y, output, x+y, strconv.FormatInt(int64(output), 2), strconv.FormatInt(int64(x+y), 2))

			for v := range initValues {
				if initValues[v] == 1 {
					fmt.Printf("\t%s=%d\n", v, initValues[v])
				}
			}
			fmt.Println()
		}
	}

	// Try to change.
	for entry := range graph {
		graph2 := make(map[string]Action)
		for k, v := range graph {
			graph2[k] = v
		}

		swapEntries(graph2, "ggk", entry)
		x := 68719476736
		y := 0
		initValues := valToBits(x, y)
		for node := range graph2 {
			if node[0] == 'z' {
				findValueOfZ(graph2, node, initValues, 0)
			}
		}

		output = computeVal(initValues, "z")
		if output == x + y {
			fmt.Println("NODE TO CHANGE", entry)
		}
	}
}

func findValueOfZ(graph map[string]Action, node string, values map[string]int, maxDepth int) int {
	if maxDepth >= 100 {
		return -1
	}
	// Get the parents of the node and the action.
	action := graph[node]

	// If the value already exists, we can fetch it, otherwise we must compute it.
	valA, okA := values[action.A]
	if !okA {
		valA = findValueOfZ(graph, action.A, values, maxDepth + 1)
	}

	// Shortcut.
	// If AND and valA is 0, set 0.
	// If OR and valA is 1, set 1.
	if valA > 0 && action.Action == "OR" {
		values[node] = 1
		return 1
	}
	if valA == 0 && action.Action == "AND" {
		values[node] = 0
		return 0
	}

	valB, okB := values[action.B]
	if !okB {
		valB = findValueOfZ(graph, action.B, values, maxDepth + 1)
	}

	var output int
	switch action.Action {
	case "XOR":
		output = valA ^ valB
	case "AND":
		output = valA & valB
	case "OR":
		output = valA | valB
	}

	values[node] = output
	return output
}

func computeVal(values map[string]int, init string) int {
	output := 0
	for i := 0; i < 64; i++ {
		var nodeName string
		if i < 10 {
			nodeName = init + "0" + fmt.Sprint(i)
		} else {
			nodeName = init + fmt.Sprint(i)
		}
		val, ok := values[nodeName]
		if !ok {
			break
		}
		output = output | (val << i)
	}
	return output
}

func valToBits(x int, y int) map[string]int {
	out := make(map[string]int)

	for i := 0; i < 45; i++ {
		var xName string
		if i < 10 {
			xName = "x0" + fmt.Sprint(i)
		} else {
			xName = "x" + fmt.Sprint(i)
		}
		out[xName] = (x >> i) & 1
	}

	for i := 0; i < 45; i++ {
		var yName string
		if i < 10 {
			yName = "y0" + fmt.Sprint(i)
		} else {
			yName = "y" + fmt.Sprint(i)
		}
		out[yName] = (y >> i) & 1
	}

	return out
}

func swapEntries(graph map[string]Action, a string, b string) {
	if a == b {
		return
	}

	tmp := graph[a]
	graph[a] = graph[b]
	graph[b] = tmp
}
