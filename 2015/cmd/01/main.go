package main

import (
	"fmt"
	"os"
)

func main() {
	input, err := os.ReadFile("./inputs/01.txt")
	if err != nil {
		panic(err)
	}
	var floor int
	var basement []int
	for i, c := range input {
		switch c {
		case '(':
			floor += 1
		case ')':
			floor -= 1
		default:
			panic("should not occur")
		}
		if floor == -1 {
			basement = append(basement, i+1)
		}
	}
	fmt.Printf("part1: %d\n", floor)
	if len(basement) > 0 {
		fmt.Printf("part2: %d\n", basement[0])
	}
}
