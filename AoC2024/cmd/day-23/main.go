package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
)

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)

	graph := make(map[string][]string)

	// Build the graph.
	for scanner.Scan() {
		s := scanner.Text()
		tab := strings.Split(s, "-")

		_, ok := graph[tab[0]]
		if ok {
			graph[tab[0]] = append(graph[tab[0]], tab[1])
		} else {
			graph[tab[0]] = []string{tab[1]}
		}

		_, ok = graph[tab[1]]
		if ok {
			graph[tab[1]] = append(graph[tab[1]], tab[0])
		} else {
			graph[tab[1]] = []string{tab[0]}
		}
	}

	// Only keep the sub-graph containing connections with a node starting with "t".
	graphT := make(map[string][]string)
	for node := range graph {
		// Whether it starts with a 't'.
		canAddNode := node[0] == 't'
		if !canAddNode {
			for _, neigh := range graph[node] {
				if neigh[0] == 't' {
					canAddNode = true
					break
				}
			}
		}
		if canAddNode {
			graphT[node] = graph[node]
		}
	}

	found := make(map[string]bool)
	total := findClique(graphT, found)
	fmt.Println(total)

	// Part 2. Find maximum size clique.
	clique := make(map[string]bool)
	var maxClique []string
	cache := make(map[string][]string)
	for node := range graphT {
		newClique := findMaxClique(graphT, clique, node, cache)
		if len(newClique) > len(maxClique) {
			maxClique = newClique
		}
	}

	sort.Strings(maxClique)
	s := strings.Join(maxClique, ",")
	fmt.Println(s)
}

func findClique(graph map[string][]string, found map[string]bool) int {
	nbFound := 0
	for node := range graph {
		neighs := graph[node]
		// Test each clique of 3 nodes, including this one.
		for i := 0; i < len(graph[node]); i++ {
			for j := 0; j < i; j++ {
				// Create a subgroup of this node and its neighbors.
				clique := make(map[string]bool)
				clique[node] = true // Add the first node.
				clique[neighs[i]] = true
				clique[neighs[j]] = true

				cliqueString := toString(clique, "")
				if found[cliqueString] {
					continue
				}

				// Now test if this clique exists.
				if checkClique(graph, clique) {
					nbFound++
					// Mark this clique as known.
					found[cliqueString] = true
				}
			}
		}

	}
	return nbFound
}

func checkClique(graph map[string][]string, clique map[string]bool) bool {
	// Assert that the clique contains a node with "t".
	foundT := false
	for k1 := range clique {
		if k1[0] == 't' {
			foundT = true
		}
		nbFound := 0
		for _, neigh := range graph[k1] {
			if clique[neigh] {
				nbFound++
			}
		}
		if nbFound != len(clique)-1 {
			return false
		}
	}
	return true && foundT
}

func toString(clique map[string]bool, a string) string {
	var keys []string
	for k := range clique {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	s := strings.Join(keys, "-")
	if a != "" {
		s += "-" + fmt.Sprint(a)
	}
	return s
}

func findMaxClique(graph map[string][]string, clique map[string]bool, i string, cache map[string][]string) []string {
	maxClique := toSlice(clique)
	cliqueCache := toString(clique, i)
	v, ok := cache[cliqueCache]
	if ok {
		return v
	}

	// Check if this is a clique.
	if checkCliqueFast(graph, clique, i) {
		// Add this node in the graph.
		clique[i] = true

		// Add all its neighbors.
		for _, nei := range graph[i] {
			if clique[nei] {
				continue
			}
			newClique := findMaxClique(graph, clique, nei, cache)

			if len(newClique) > len(maxClique) {
				maxClique = newClique
			}
		}

		delete(clique, i)
	}

	cache[cliqueCache] = maxClique

	return maxClique
}

func checkCliqueFast(graph map[string][]string, clique map[string]bool, i string) bool {
	// Check if adding this node `i` keeps it a clique.
	nbFound := 0
	for _, nei := range graph[i] {
		if clique[nei] {
			nbFound++
		}
	}
	return len(clique) == nbFound
}

func toSlice(clique map[string]bool) []string {
	var out []string
	for k := range clique {
		out = append(out, k)
	}
	return out
}
