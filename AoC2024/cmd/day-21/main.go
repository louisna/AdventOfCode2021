package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)

	cache := make(map[string]int)

	total := 0

	for scanner.Scan() {
		s := scanner.Text()
		command := searchNumber(s, cache)

		// Compute complexity.
		sInt := s[:len(s)-1]
		v, _ := strconv.Atoi(sInt)
		total += v * command
	}
	fmt.Println(total)
}

func searchNumber(command string, cache map[string]int) int {
	numLoc := map[rune][2]int{'A': {0, 0}, '0': {0, 1}, '1': {1, 2}, '2': {1, 1}, '3': {1, 0}, '4': {2, 2}, '5': {2, 1}, '6': {2, 0}, '7': {3, 2}, '8': {3, 1}, '9': {3, 0}}
	loc := numLoc['A']

	total := 0
	previousDigit := 'A'

	for _, nextDigit := range command {
		nextLoc := numLoc[nextDigit]

		cacheDigit := string(nextDigit) + "-B-" + string(previousDigit)
		val, okD := cache[cacheDigit]
		if okD {
			total += val

			previousDigit = nextDigit
			loc = nextLoc
			continue
		}

		// This contains all the possible permutations to go from "loc" to "nextLoc".
		allPaths := generatePaths(loc, nextLoc, true)

		// Iterate over all these paths to find the one with the lowest sequence length.
		minValue := 1000000000000
		for _, candidate := range allPaths {
			cacheCandidate := candidate + "-B"
			commandThisCandidate, ok := cache[cacheCandidate]
			// If not already present, we compute the path, then we will get the minimum.
			if !ok {
				shortestSeqThisCandidate := searchPattern(candidate, cache, 25)
				commandThisCandidate = shortestSeqThisCandidate
			}

			if commandThisCandidate < minValue {
				minValue = commandThisCandidate
			}
		}

		cache[cacheDigit] = minValue
		total += minValue

		previousDigit = nextDigit
		loc = nextLoc
	}

	// Returns the string as concatenation of all the shortest paths.
	return total
}

func searchPattern(command string, cache map[string]int, depth int) int {
	if depth == 0 {
		return len(command)
	}

	total := 0

	digitLoc := map[rune][2]int{'A': {1, 0}, '>': {0, 0}, 'v': {0, 1}, '<': {0, 2}, '^': {1, 1}}
	loc := digitLoc['A']
	previousDigit := 'A'

	for _, nextDigit := range command {
		nextLoc := digitLoc[nextDigit]
		cacheDigit := string(nextDigit) + "-" + string(previousDigit) + "-" + fmt.Sprint(depth)
		val, okD := cache[cacheDigit]
		if okD {
			total += val

			previousDigit = nextDigit
			loc = nextLoc
			continue
		}

		// This contains all the possible permutations to go from "loc" to "nextLoc".
		allPaths := generatePaths(loc, nextLoc, false)

		// Iterate over all these paths to find the one with the lowest sequence length.
		minValue := 1000000000000
		for _, candidate := range allPaths {
			commandThisCandidate, ok := cache[candidate]
			// If not already present, we compute the path, then we will get the minimum.
			if !ok {
				shortestSeqThisCandidate := searchPattern(candidate, cache, depth-1)
				commandThisCandidate = shortestSeqThisCandidate
			}

			if commandThisCandidate < minValue {
				minValue = commandThisCandidate
			}
		}

		if len(allPaths) > 0 {
			cache[cacheDigit] = minValue
			total += minValue
		} else {
			total += 1
		}

		loc = nextLoc
		previousDigit = nextDigit
	}
	return total
}

func permutations(arr []string) [][]string {
	var helper func([]string, int)
	res := [][]string{}

	helper = func(arr []string, n int) {
		if n == 1 {
			tmp := make([]string, len(arr))
			copy(tmp, arr)
			res = append(res, tmp)
		} else {
			for i := 0; i < n; i++ {
				helper(arr, n-1)
				if n%2 == 1 {
					tmp := arr[i]
					arr[i] = arr[n-1]
					arr[n-1] = tmp
				} else {
					tmp := arr[0]
					arr[0] = arr[n-1]
					arr[n-1] = tmp
				}
			}
		}
	}
	helper(arr, len(arr))
	return res
}

func generatePaths(loc [2]int, nextLoc [2]int, isDigit bool) []string {
	var firstPerm bytes.Buffer
	if loc[0] < nextLoc[0] {
		for i := loc[0]; i < nextLoc[0]; i++ {
			firstPerm.WriteString("^")
		}
	} else {
		for i := nextLoc[0]; i < loc[0]; i++ {
			firstPerm.WriteString("v")
		}
	}

	if loc[1] < nextLoc[1] {
		for i := loc[1]; i < nextLoc[1]; i++ {
			firstPerm.WriteString("<")
		}
	} else {
		for i := nextLoc[1]; i < loc[1]; i++ {
			firstPerm.WriteString(">")
		}
	}
	firstPermS := firstPerm.String()
	allPaths := permutations(strings.Split(firstPermS, ""))

	// Filter out duplicate permutations.
	permutations := make(map[string]bool)
	for _, path := range allPaths {
		pathS := strings.Join(path, "")

		// Check if the permutation goes out of bounds.
		// For this, we compute the path that the robot follows and disable this permutation if it goes out of the box.
		nextI, nextJ := loc[0], loc[1]
		canAdd := true
		for _, s := range path {
			if s == "<" {
				nextJ++
			} else if s == ">" {
				nextJ--
			} else if s == "^" {
				nextI++
			} else if s == "v" {
				nextI--
			}
			if (isDigit && nextI == 0 && nextJ == 2) || (!isDigit && nextI == 1 && nextJ == 2) {
				canAdd = false
				break
			}
		}

		if canAdd {
			permutations[pathS] = true
		}
	}

	// For each new permutation, compute the path towards us controlling it.
	var keys []string
	for perm := range permutations {
		permSequence := perm + "A"

		keys = append(keys, permSequence)
	}
	return keys
}
