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

	var allKeys [][]int
	var allLocks [][]int

	tmp := make(map[int]int)
	isKey := false
	firstRead := true
	for scanner.Scan() {
		s := scanner.Text()
		
		// Space, restart the process.
		if len(s) < 4 {
			// Convert to slice format.
			sliceFormat := make([]int, 5)
			for i := 0; i < 5; i++ {
				sliceFormat[i] = tmp[i]
			}

			if isKey {
				allKeys = append(allKeys, sliceFormat)
			} else {
				allLocks = append(allLocks, sliceFormat)
			}
			tmp = make(map[int]int)
			firstRead = true
			continue
		}

		// Whether it is a key or a lock.
		if firstRead {
			if s[0] == '#' {
				isKey = false
			} else {
				isKey = true
			}
			firstRead = false
		}

		for i, b := range s {
			if b == '#' {
				v, ok := tmp[i]
				if ok {
					tmp[i] = v + 1
				} else {
					tmp[i] = 0
				}
			}
		}
	}

	fmt.Println(allKeys)
	fmt.Println(allLocks)

	total := 0

	// Used keys.
	used := make(map[[2]int]bool)

	// Try all pairs.
	for i, key := range allKeys {
		for j, lock := range allLocks {
			if used[[2]int{i, j}] {
				continue
			}
			isSame := true
			for k := 0; k < 5; k++ {
				if key[k] + lock[k] > 5 {
					isSame = false
					break
				}
			}
			if isSame {
				used[[2]int{i, j}] = true
				total++
			}
		}
	}

	fmt.Println(total)
}
