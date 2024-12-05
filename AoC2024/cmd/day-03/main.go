package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)

	// Get the line.
	scanner.Scan()
	s := scanner.Text()

	r := regexp.MustCompile(`mul\((\d+),(\d+)\)|do\(\)|don't\(\)`)
	matches := r.FindAllStringSubmatch(s, -1)
	
	res := 0
	do := true
	for _, match := range matches {
		if match[0] == "do()" {
			do = true
		} else if match[0] == "don't()" {
			do = false
		} else if do {
			v1, _ := strconv.Atoi(match[1])
			v2, _ := strconv.Atoi(match[2])
			res += v1 * v2
		}
	}

	fmt.Println(res)
}
