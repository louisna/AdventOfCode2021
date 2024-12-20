package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	filename := os.Args[1]

	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)

	// Maintain the elements inside a map instead.
	elems := make(map[byte][][]int)

	// Maintain a set of all positions.
	positions := make(map[[2]int]bool)

	i := 0
	for scanner.Scan() {
		s := scanner.Text()
		for j := 0; j < len(s); j++ {
			// Skip where nothing.
			if s[j] == '.' {
				continue
			}

			// Create the inner map if it does not exist.
			_, ok := elems[s[j]]
			if ok {
				elems[s[j]] = append(elems[s[j]], []int{i, j})
			} else {
				elems[s[j]] = [][]int{{i, j}}
			}
		}
		i++
	}

	nbPoints := 0

	// Create pairs for each byte.
	for _, beacons := range elems {
		// Iter two times to create all the pairs.
		for b1 := 0; b1 < len(beacons); b1++ {
			for b2 := b1 + 1; b2 < len(beacons); b2++ {
				beacon1 := beacons[b1]
				beacon2 := beacons[b2]

				// There are two new points from these beacons.
				newPoints := computePoints(beacon1[0], beacon1[1], beacon2[0], beacon2[1], i)

				for idx := range newPoints {
					// Check that still in the grid.
					if newPoints[idx][0] < 0 || newPoints[idx][0] >= i || newPoints[idx][1] < 0 || newPoints[idx][1] >= i {
						continue
					}

					if !positions[[2]int{newPoints[idx][0], newPoints[idx][1]}] {
						positions[[2]int{newPoints[idx][0], newPoints[idx][1]}] = true
						nbPoints++
					}
				}
			}
		}
	}

	fmt.Println(nbPoints)
}

func computePoints(i0 int, j0 int, i1 int, j1 int, n int) [][2]int {
	deltaI := i0 - i1
	deltaJ := j0 - j1
	var newNodes [][2]int
	
	for i := 0; i < 100; i++ {
		if i0 + deltaI * i >= 0 && i0 + deltaI * i < n && j0 + deltaJ * i >= 0 && j0 + deltaJ * i < n {
			newNodes = append(newNodes, [2]int{i0 + deltaI * i, j0 + deltaJ * i})
		} else {
			break
		}	
	}

	for i := 0; i < 100; i++ {
		if i1 - deltaI * i >= 0 && i1 - deltaI * i < n && j1 - deltaJ * i >= 0 && j1 - deltaJ * i < n {
			newNodes = append(newNodes, [2]int{i1 - deltaI * i, j1 - deltaJ * i})
		} else {
			break
		}	
	}

	return newNodes
}
