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

	nbTokens := 0
	r := regexp.MustCompile(`Button ([A-Z]): X\+(\d+), Y\+(\d+)|Prize: X=(\d+), Y=(\d+)`)

	var aX, aY, bX, bY, pX, pY int
	for scanner.Scan() {

		s := scanner.Text()
		matchesAll := r.FindAllStringSubmatch(s, -1)

		if len(matchesAll) == 0 {
			a, b := computeDist(aX, aY, bX, bY, pX, pY)
			if a*aX+b*bX == pX && a*aY+b*bY == pY {
				nbTokens += a*3 + b
			}
			continue
		}

		matches := matchesAll[0]

		if matches[0][0] != 'P' {
			// Buttons.
			if matches[1] == "A" {
				aX, _ = strconv.Atoi(matches[2])
				aY, _ = strconv.Atoi(matches[3])
			} else {
				bX, _ = strconv.Atoi(matches[2])
				bY, _ = strconv.Atoi(matches[3])
			}
		} else {
			pX, _ = strconv.Atoi(matches[4])
			pX = pX + 10000000000000
			pY, _ = strconv.Atoi(matches[5])
			pY = pY + 10000000000000
		}
	}

	// Do it a last time for the last input.
	a, b := computeDist(aX, aY, bX, bY, pX, pY)
	if a*aX+b*bX == pX && a*aY+b*bY == pY {
		nbTokens += a*3 + b
	}

	fmt.Println(nbTokens)
}

func computeDist(aX int, aY int, bX int, bY int, pX int, pY int) (int, int) {
	b := (aX*pY - pX*aY) / (aX*bY - bX*aY)
	a := (pX - b*bX) / (aX)
	return a, b
}
