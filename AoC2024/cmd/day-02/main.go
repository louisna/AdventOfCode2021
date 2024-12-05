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

	nb_safe := 0

	for scanner.Scan() {
		s := scanner.Text()
		tab := strings.Split(s, " ")

		// To simplify, we will make both iterations considering increasing and decreasing.
		// And we take the minimum of the two.
		line_is_safe := false
		for idx_to_remove := -1; idx_to_remove < len(tab); idx_to_remove++ {
			for mode := 0; mode < 2; mode++ {
				has_errors := 0

				prev, _ := strconv.Atoi(tab[0])
				if idx_to_remove == 0 {
					prev, _ = strconv.Atoi(tab[1])
				}

				for i := 1; i < len(tab); i++ {
					if i == idx_to_remove {
						continue
					}

					if idx_to_remove == 0 && i == 1 {
						continue
					}

					val, _ := strconv.Atoi(tab[i])

					if mode == 0 && (val <= prev || val - prev > 3) {
						has_errors++
					}

					if mode == 1 && (val >= prev || prev - val > 3) {
						has_errors++
					}

					prev = val
				}

				if has_errors == 0 {
					line_is_safe = true
					nb_safe++
					break
				}
			}

			if line_is_safe {
				break
			}
		}
	}

	fmt.Println(nb_safe)
}
