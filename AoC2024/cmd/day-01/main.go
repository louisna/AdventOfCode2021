package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)
	scanner := bufio.NewScanner(file)

	nb_lines := 0

	for scanner.Scan() {
		nb_lines++
	}

	// Now the content.
	file, _ = os.Open(filename)
	scanner = bufio.NewScanner(file)

	// And array definition.
	left := make([]int, nb_lines)
	right := make([]int, nb_lines)

	i := 0
	for scanner.Scan() {
		s := scanner.Text()
		tab := strings.Split(s, "   ")
		v1, _ := strconv.Atoi(tab[0])
		v2, _ := strconv.Atoi(tab[1])
		left[i] = v1
		right[i] = v2
		i++
	}

	sort.Ints(left)
	sort.Ints(right)

	// Compute the difference.
	total_diff := 0
	for i := range left {
		v1 := left[i]
		v2 := right[i]
		if v1 > v2 {
			total_diff += v1 - v2
		} else {
			total_diff += v2 - v1
		}
	}

	fmt.Println(total_diff)

	// Second part.
	sim := 0
	for i := range left {
		nb := 0
		for j := range right {
			if left[i] == right[j] {
				nb++
			}
		}
		sim += left[i] * nb
	}

	fmt.Println(sim)
}
