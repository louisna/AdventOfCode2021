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

	var patterns []string
	var targets []string

	readTarget := false
	for scanner.Scan() {
		s := scanner.Text()

		if len(s) == 0 {
			readTarget = true
			continue
		}
		
		if readTarget {
			targets = append(targets, s)
		} else {
			tab := strings.Split(s, ", ")
			patterns = tab
		}
	}

	total := 0
	totalPermutations := 0
	cache := make(map[string]int)
	for _, target := range targets {
		nb := treeSearch(patterns, target, cache)
		if nb > 0 {
			total++
			totalPermutations += nb
			fmt.Println(target)
		}
	}

	fmt.Println(total)
	fmt.Println(totalPermutations)
}

func treeSearch(patterns []string, word string, cache map[string]int) int {
	possible := 0

	if len(word) == 0 {
		return 1
	}

	v, ok := cache[word]
	if ok {
		return v
	}

	for _, pattern := range patterns {
		// Check if the sub-pattern fits inside the word.
		if len(pattern) > len(word) {
			continue
		}

		if pattern == word[:len(pattern)] {
			// This fits inside!
			nb := treeSearch(patterns, word[len(pattern):], cache)
			if nb > 0 {
				possible += nb
			}
		}
	}
	
	// Cannot find one, so it's not possible with the substring we got.
	cache[word] = possible
	return possible
}
