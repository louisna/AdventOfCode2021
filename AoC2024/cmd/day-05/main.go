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

	total := 0
	totalCorrected := 0
	dependencies:= make(map[int]map[int]bool)

	// First the dependencies.
	for scanner.Scan() {
		s := scanner.Text()
		
		if len(s) < 2 {
			break
		}

		
		tab := strings.Split(s, "|")
		from, _ := strconv.Atoi(tab[0])
		to, _ := strconv.Atoi(tab[1])
		
		if dependencies[from] == nil {
			dependencies[from] = map[int]bool{}
		}
		
		dependencies[from][to] = true
	}

	// Then the order.
	for scanner.Scan() {
		s := scanner.Text()

		tab := strings.Split(s, ",")

		// Convert to integer all.
		tab_int := make([]int, len(tab))
		for i := 0; i < len(tab); i++ {
			val, _ := strconv.Atoi(tab[i])
			tab_int[i] = val
		}

		// Iter to find a preceding order.
		valid := true
		for i := 1; i < len(tab); i++ {
			for j := 0; j < i; j++ {
				// Check if key exists.
				_, ok := dependencies[tab_int[i]][tab_int[j]]
				if ok {
					valid = false
					break
				}
			}
			if !valid {
				break
			}
		}

		if valid {
			total += tab_int[len(tab_int) / 2]
			continue
		}

		// Try to correct the order if it is not valid.
		for ;; {
			valid = true
			for i := 1; i < len(tab); i++ {
				for j := 0; j < i; j++ {
					// Check if key exists.
					_, ok := dependencies[tab_int[i]][tab_int[j]]
					if ok {
						tmp := tab_int[i]
						tab_int[i] = tab_int[j]
						tab_int[j] = tmp
					}
				}
			}

			if valid {
				totalCorrected += tab_int[len(tab_int) / 2]
				break
			}
		}
	}

	fmt.Println(total)
	fmt.Println(totalCorrected)
}
