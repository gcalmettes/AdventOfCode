package main

import (
	"fmt"
	"os"
)

const (
	UP    byte = '^'
	DOWN  byte = 'v'
	LEFT  byte = '<'
	RIGHT byte = '>'
)

type Pos struct {
	x int
	y int
}
type Grid map[Pos]int

func main() {
	input, err := os.ReadFile("./inputs/03.txt")
	if err != nil {
		panic(err)
	}

	g1 := make(Grid)
	g2 := make(Grid)

	// starting position
	current := &Pos{0, 0}
	currents := []*Pos{{0, 0}, {0, 0}}

	g1[*current] = 1
	g2[*current] = 2

	for i, dir := range input {
		p := 1
		if i%2 == 0 {
			p = 0
		}

		switch dir {
		case UP:
			current.y -= 1
			currents[p].y -= 1
		case DOWN:
			current.y += 1
			currents[p].y += 1
		case LEFT:
			current.x -= 1
			currents[p].x -= 1
		case RIGHT:
			current.x += 1
			currents[p].x += 1
		default:
			panic("should not be here")
		}

		g1[*current] += 1
		for _, g := range currents {
			g2[*g] += 1
		}
	}
	fmt.Printf("part1: %d\n", len(g1))
	fmt.Printf("part1: %d\n", len(g2))
}
