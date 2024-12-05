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

	var array [][]string

	for scanner.Scan() {
		s := scanner.Text()
		tab := strings.Split(s, "")
		array = append(array, tab)
	}

	total := 0

	for i := 0; i < len(array); i++ {
		for j := 0; j < len(array[0]) - 3; j++ {
			// Construct word horizontally.
			s := array[i][j] + array[i][j + 1] + array[i][j + 2] + array[i][j + 3]
			if s == "XMAS" || s == "SAMX" {
				total++
			}

			// Construct word vertically.
			s = array[j][i] + array[j + 1][i] + array[j + 2][i] + array[j + 3][i]
			if s == "XMAS" || s == "SAMX" {
				total++
			}

			// Construct word with diagonal.
			if i < len(array) - 3 {
				s = array[i][j] + array[i + 1][j + 1] + array[i + 2][j + 2] + array[i + 3][j + 3]
				if s == "XMAS" || s == "SAMX" {
					total++
				}
			}

			// Construct word with diagonal.
			if i >= 3 {
				s = array[i][j] + array[i - 1][j + 1] + array[i - 2][j + 2] + array[i - 3][j + 3]
				if s == "XMAS" || s == "SAMX" {
					total++
				}
			}
		} 
	}

	fmt.Println(total)

	total2 := 0

	for i := 0; i < len(array) - 2; i++ {
		for j := 0; j < len(array) - 2; j++ {
			s1 := array[i][j] + array[i + 1][j + 1] + array[i + 2][j + 2]
			s2 := array[i + 2][j] + array[i + 1][j + 1] + array[i][j + 2]
			if (s1 == "MAS" || s1 == "SAM") && (s2 == "MAS" || s2 == "SAM") {
				total2++
			}
		}
	}

	fmt.Println(total2)
}
