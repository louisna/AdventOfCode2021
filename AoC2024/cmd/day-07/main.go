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

	sumOk := 0

	for scanner.Scan() {
		s := scanner.Text()
		tab := strings.Split(s, " ")
		expected, _ := strconv.Atoi(tab[0][:len(tab[0])-1])
		values := make([]int, len(tab)-1)
		for i := 1; i < len(tab); i++ {
			values[i - 1], _ = strconv.Atoi(tab[i])
		}

		if isPossibleAcc(expected, values, 1, values[0]) {
			sumOk += expected
		}
	}

	fmt.Println(sumOk)
}

func isPossibleAcc(expected int, values []int, index int, acc int) bool {
	if index == len(values) {
		return expected == acc
	}

	// Early stop.
	if expected < acc {
		return false
	}

	// Multiplication.
	newAcc := acc * values[index]
	if isPossibleAcc(expected, values, index+1, newAcc) {
		return true
	}

	// Concatenation.
	leftString := strconv.Itoa(acc)
	rightString := strconv.Itoa(values[index])
	concatString := leftString + rightString
	newAcc, _ = strconv.Atoi(concatString)
	if isPossibleAcc(expected, values, index+1, newAcc) {
		return true
	}

	// Sum.
	newAcc = acc + values[index]
	return isPossibleAcc(expected, values, index+1, newAcc)
}
