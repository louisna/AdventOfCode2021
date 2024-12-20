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

	// Read and convert to integer all values.
	scanner.Scan()
	s := scanner.Text()
	valuesString := strings.Split(s, "")
	
	// Contains all values.
	var values [][2]int

	// Whether this is empty space.
	isFree := false

	// Index of the block.
	index := 0

	// Create the table of values.
	for i := range valuesString {
		v, _ := strconv.Atoi(valuesString[i])
		
		// Iter, giving an ID to each number.
		for j := 0; j < v; j++ {
			if isFree {
				values = append(values, [2]int{-1, v})
				} else {
				values = append(values, [2]int{index,v})
			}
		}

		if !isFree {
			index++
		}

		isFree = !isFree
	}

	// Replacing from the end.
	idxStart := 0
	idxEnd := len(values) - 1

	// For part 2.
	valuesPart2 := make([][2]int, len(values))
	for i := range values {
		valuesPart2[i] = [2]int{values[i][0], values[i][1]}
	}

	// Result.
	sum := 0

	for ; idxStart < idxEnd ; {
		// Advance the pointer until next free space.
		if values[idxStart][0] > -1 {
			// Advance sum.
			sum += values[idxStart][0] * idxStart

			idxStart++
			continue
		}

		// Back the other pointer until previous non-empty space.
		if values[idxEnd][0] == -1 {
			idxEnd--
			continue
		}

		// Replace the value to the beginnin.
		values[idxStart][0] = values[idxEnd][0]
		values[idxEnd][0] = -1
	}

	fmt.Println(sum)

	// Part 2.
	values = valuesPart2

	for j := len(values) - 1; j >= 0; j-- {
		// Skip enmpty space.
		if values[j][0] == -1 {
			continue
		}

		val := values[j][0]
		howMuch := values[j][1]

		// Try to find an empty space from the beginning.
		for i := 0; i < j; i++ {
			// Skip full space.
			if values[i][0] > -1 {
				continue
			}

			howMuchEmpty := values[i][1]
			if howMuchEmpty >= howMuch {
				// Can move!
				for k := 0; k < howMuch; k++ {
					// Add in the empty space.
					values[i + k][0] = val
					values[i + k][1] = howMuch

					// Remove from old place.
					values[j - k][0] = -1
					// No need to update how much.
				}

				// Left empty space must also be updated.
				for k := howMuch; k < howMuchEmpty; k++ {
					values[i + k][1] -= howMuch
				}
				break
			}
		}
	}

	// Compute the checksum.
	checksum := 0

	for i := range values {
		fmt.Printf("(%d,%d) ", values[i][0], values[i][1])
		if values[i][0] > -1 {
			checksum += values[i][0] * i
		}
	}

	fmt.Println()

	fmt.Println(checksum)

	
}
