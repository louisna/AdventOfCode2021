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

	// All registers.
	registers := make([]int, 3)

	// List of opcodes and operands.
	var opcodes []int

	i := 0
	for scanner.Scan() {
		s := scanner.Text()

		if i < 3 {
			tab := strings.Split(s, ": ")
			v, _ := strconv.Atoi(tab[1])
			registers[i] = v
		} else if i == 4 {
			tab := strings.Split(s, ": ")
			tab2 := strings.Split(tab[1], ",")
			for _, elem := range tab2 {
				v, _ := strconv.Atoi(elem)
				opcodes = append(opcodes, v)
			}
		}
		i++
	}

	out := execute(registers, opcodes)

	// Print the output.
	for _, v := range(out) {
		fmt.Printf("%d,", v)
	}

	fmt.Println()
	fmt.Println("Part 2")
	for i := 0; i < 8; i++ {
		fmt.Println(part2(opcodes, i, 0))

	}
}

func execute(registers []int, opcodes []int) []int {
	var out []int
	
	opIdx := 0

	// Execute the computer.
	for {
		if opIdx >= len(opcodes) {
			break
		}

		// Get opcode and operand.
		opcode := opcodes[opIdx]
		operand := opcodes[opIdx+1]

		// Execute the operation on the registers.
		switch opcode {
		case 0:
			registers[0] = int(math.Floor((float64(registers[0]) / math.Pow(2, float64(combo(registers, operand))))))
		case 1:
			registers[1] = registers[1] ^ operand
		case 2:
			registers[1] = combo(registers, operand) % 8
		case 3:
			if registers[0] != 0 {
				opIdx = operand
				continue
			}
		case 4:
			registers[1] = registers[1] ^ registers[2]
		case 5:
			out = append(out, combo(registers, operand) % 8)
		case 6:
			registers[1] = int(math.Floor((float64(registers[0]) / math.Pow(2, float64(combo(registers, operand))))))
		case 7:
			registers[2] = int(math.Floor((float64(registers[0]) / math.Pow(2, float64(combo(registers, operand))))))
		}

		// Update opcode index.
		opIdx += 2
	}

	// fmt.Println(registers)

	return out
}

func combo(registers []int, operand int) int {
	opValue := operand
	if operand > 3 {
		opValue = registers[operand-4]
	}
	return opValue
}

func part2(opcodes []int, a int, i int) []int {
	registers := []int{a,0,0}
	res := execute(registers, opcodes)
	if compareSlices(res, opcodes) {
		fmt.Println("Found!", res, opcodes, a)
		return []int{a,0,0}
	}
	registers = []int{a,0,0}
	if compareSlices(execute(registers, opcodes), opcodes[len(opcodes) - 1 - i:]) {
		for n := 0; n < 8; n++ {
			part2(opcodes, 8*a+n, i+1)
		}
	}

	return nil
}

func compareSlices(a []int, b []int) bool {
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}