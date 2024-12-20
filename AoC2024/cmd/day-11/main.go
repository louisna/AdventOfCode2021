package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)
	scanner.Scan()

	s := scanner.Text()
	tab := strings.Split(s, " ")

	values := make([]int, len(tab))
	for i := 0; i < len(tab); i++ {
		v, _ := strconv.Atoi(tab[i])
		values[i] = v
	}

	cache := make(map[[2]int]int)

	total := 0
	for _, v := range values {
		total += dfs(v, 75, cache)
	}

	fmt.Println()
	fmt.Println(total)
}

func dfs(val int, iterLeft int, cache map[[2]int]int) int {
	if iterLeft == 0 {
		return 1
	}

	cacheVal, ok := cache[[2]int{val, iterLeft}]
	if ok {
		return cacheVal
	}

	var newVal int
	if val == 0 {
		// Rule 1.
		newVal = dfs(1, iterLeft - 1, cache)
	} else {
		nbDigit := int64(math.Log10(float64(val))) + 1

		// Rule 2.
		if nbDigit % 2 == 0 {
			left := val / int(math.Pow10(int(nbDigit) / 2))
			right := val - left * int(math.Pow10(int(nbDigit / 2)))

			newVal = dfs(left, iterLeft - 1, cache) + dfs(right, iterLeft - 1, cache)
		} else {
			// Rule 3.
			newVal = dfs(val * 2024, iterLeft - 1, cache)
		}
	}
	cache[[2]int{val, iterLeft}] = newVal
	return newVal
}
