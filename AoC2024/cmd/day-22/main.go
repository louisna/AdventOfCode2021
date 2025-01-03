package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)

	total := 0

	// New prices.
	var updates [][]int

	// Variations in prices.
	var variations [][]int

	for scanner.Scan() {
		s := scanner.Text()
		v, _ := strconv.Atoi(s)
		lastVariation := v % 10

		var updatesThisMonkey []int
		var variationsThisMonkey []int

		updatesThisMonkey = append(updatesThisMonkey, v%10)
		variationsThisMonkey = append(variationsThisMonkey, 0)

		for i := 0; i < 2000; i++ {
			v = applyOnce(v)
			updatesThisMonkey = append(updatesThisMonkey, v%10)
			variationsThisMonkey = append(variationsThisMonkey, v%10-lastVariation)
			lastVariation = v % 10
		}

		// Part 1.
		total += v

		updates = append(updates, updatesThisMonkey)
		variations = append(variations, variationsThisMonkey)
	}

	// Part 1.
	fmt.Println(total)

	// Fill, for each monkey, the patterns and prices we would get.
	prices := make(map[[4]int]int)
	for i := 0; i < len(updates); i++ {
		fillVariationOccurrences(prices, updates[i], variations[i])
	}

	// Get the pattern with the most bananas.
	maxBananas := 0
	var bestPattern [4]int
	for k := range prices {
		v := prices[k]
		if v > maxBananas {
			maxBananas = v
			bestPattern = k
		}
	}

	fmt.Println(maxBananas, bestPattern)
}

func applyOnce(secret int) int {
	tmp := secret * 64
	secret = prune(mix(tmp, secret))

	tmp = int(math.Floor(float64(secret) / 32.0))
	secret = prune(mix(tmp, secret))

	tmp = secret * 2048
	secret = prune(mix(tmp, secret))

	return secret
}

func mix(val int, secretNumber int) int {
	return val ^ secretNumber
}

func prune(val int) int {
	return val % 16777216
}

func fillVariationOccurrences(prices map[[4]int]int, priceEvolution []int, variations []int) {
	// Because we only focus on the first occurence 
	// of each pattern on each monkey, we must also
	// avoid repeating the same pattern twice for the same monkey.
	variationsMonkey := make(map[[4]int]bool)

	for i := 1; i < len(variations) - 4; i++ {
		// Get the 4-prices pattern.
		pattern := [4]int{variations[i], variations[i+1], variations[i+2], variations[i+3]}

		// Ensure that we don't count twice the same pattern for the same monkey.
		if variationsMonkey[pattern] {
			continue
		}
		variationsMonkey[pattern] = true

		price := priceEvolution[i + 3]
		
		// Check if existing value.
		v, ok := prices[pattern]
		if ok {
			prices[pattern] = v + price
		} else {
			prices[pattern] = price
		}
	}
}
