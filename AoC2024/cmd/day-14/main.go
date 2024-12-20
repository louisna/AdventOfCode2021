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
	nbX, _ := strconv.Atoi(os.Args[2])
	nbY, _ := strconv.Atoi(os.Args[3])

	file, _ := os.Open(filename)
	scanner := bufio.NewScanner(file)

	nbRobots := make([]int, 4)

	r := regexp.MustCompile(`p=(-?\d*),(-?\d*) v=(-?\d*),(-?\d*)`)

	var allX, allY, allVx, allVy []int

	for scanner.Scan() {
		s := scanner.Text()
		matches := r.FindStringSubmatch(s)

		x, _ := strconv.Atoi(matches[1])
		y, _ := strconv.Atoi(matches[2])
		vx, _ := strconv.Atoi(matches[3])
		vy, _ := strconv.Atoi(matches[4])

		allX = append(allX, x)
		allY = append(allY, y)
		allVx = append(allVx, vx)
		allVy = append(allVy, vy)
	}

	nbTurns := 100

	for i := 0; i < len(allX); i++ {
		x := allX[i]
		y := allY[i]
		vx := allVx[i]
		vy := allVy[i]

		x = (x + (vx*nbTurns)%nbX + nbX) % nbX
		y = (y + (vy*nbTurns)%nbY + nbY) % nbY

		// Determine the quadran of this robot.
		if x < nbX/2 && y < nbY/2 {
			nbRobots[0]++
		} else if x < nbX/2 && y > nbY/2 {
			nbRobots[1]++
		} else if x > nbX/2 && y > nbY/2 {
			nbRobots[2]++
		} else if x > nbX/2 && y < nbY/2 {
			nbRobots[3]++
		}
	}
	total := nbRobots[0] * nbRobots[1] * nbRobots[2] * nbRobots[3]

	fmt.Println(total)

	// Part 2.
	for nbTurn := 0; nbTurn < 10000; nbTurn++ {
		// Create grid to show the christmas tree.
		grid := make([][]bool, nbX)
		for i := range grid {
			grid[i] = make([]bool, nbY)
		}

		canContinue := true

		nbRobots := make([]int, 4)

		for i := 0; i < len(allX); i++ {
			x := allX[i]
			y := allY[i]
			vx := allVx[i]
			vy := allVy[i]

			x = (x + (vx*nbTurn)%nbX + nbX) % nbX
			y = (y + (vy*nbTurn)%nbY + nbY) % nbY

			// Determine the quadran of this robot.
			if x < nbX/2 && y < nbY/2 {
				nbRobots[0]++
			} else if x < nbX/2 && y > nbY/2 {
				nbRobots[1]++
			} else if x > nbX/2 && y > nbY/2 {
				nbRobots[2]++
			} else if x > nbX/2 && y < nbY/2 {
				nbRobots[3]++
			}

			if grid[x][y] {
				canContinue = false
				break
			}

			grid[x][y] = true
		}

		if canContinue {
			fmt.Println(nbTurn)
			for i := 0; i < nbX; i++ {
				for j := 0; j < nbY; j++ {
					if grid[i][j] {
						fmt.Print("@")
					} else {
						fmt.Print(" ")
					}
				}
				fmt.Println()
			}
			fmt.Println()
		}
	}
}
