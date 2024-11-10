package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Grid map[string]bool

func (g *Grid) GetNeighbors(coord string) []string {
	s := strings.SplitN(coord, ",", 2)
	col, _ := strconv.Atoi(s[0])
	row, _ := strconv.Atoi(s[1])
	nb := []string{}
	// squares
	if col > 0 {
		nb = append(nb, fmt.Sprintf("%d,%d", col-1, row))
	}
	if col < 99 {
		nb = append(nb, fmt.Sprintf("%d,%d", col+1, row))
	}
	if row > 0 {
		nb = append(nb, fmt.Sprintf("%d,%d", col, row-1))
	}
	if row < 99 {
		nb = append(nb, fmt.Sprintf("%d,%d", col, row+1))
	}
	// diagonals
	if col > 0 && row > 0 {
		nb = append(nb, fmt.Sprintf("%d,%d", col-1, row-1))
	}
	if col < 99 && row < 99 {
		nb = append(nb, fmt.Sprintf("%d,%d", col+1, row+1))
	}
	if col > 0 && row < 99 {
		nb = append(nb, fmt.Sprintf("%d,%d", col-1, row+1))
	}
	if col < 99 && row > 0 {
		nb = append(nb, fmt.Sprintf("%d,%d", col+1, row-1))
	}
	return nb
}

func (g Grid) ShouldBeOn(coord string) bool {
	nb := g.GetNeighbors(coord)
	countNbOn := 0
	for _, n := range nb {
		if _, isOn := g[n]; isOn {
			countNbOn += 1
		}
	}

	if _, isOn := g[coord]; isOn && (countNbOn == 2 || countNbOn == 3) {
		return true
	}

	if _, isOn := g[coord]; !isOn && countNbOn == 3 {
		return true
	}
	return false
}

func main() {
	input, err := os.ReadFile("./inputs/18.txt")
	if err != nil {
		panic(err)
	}

	specialCoords := []string{"0,0", "0,99", "99,0", "99,99"}

	grid1 := Grid{}
	grid2 := Grid{}

	for row, line := range strings.Split(string(input), "\n") {
		if len(line) == 0 {
			continue
		}
		for col := range len(line) {
			if line[col] == '#' {
				grid1[fmt.Sprintf("%d,%d", col, row)] = true
				grid2[fmt.Sprintf("%d,%d", col, row)] = true
			}
		}
	}

	for _, sc := range specialCoords {
		grid2[sc] = true
	}

	newGrid1 := Grid{}
	newGrid2 := Grid{}

	for _ = range 100 {
		for r := range 100 {
			for c := range 100 {
				coord := fmt.Sprintf("%d,%d", c, r)
				if grid1.ShouldBeOn(coord) {
					newGrid1[coord] = true
				}
				if grid2.ShouldBeOn(coord) {
					newGrid2[coord] = true
				}
			}
		}
		grid1 = newGrid1
		newGrid1 = Grid{}

		for _, sc := range specialCoords {
			newGrid2[sc] = true
		}
		grid2 = newGrid2
		newGrid2 = Grid{}
	}

	p1 := len(grid1)
	p2 := len(grid2)

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
