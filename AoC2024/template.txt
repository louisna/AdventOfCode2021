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

	for scanner.Scan() {
		s := scanner.Text()
		fmt.Println(s)
	}
}
