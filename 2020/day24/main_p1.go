package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

func main() {
	tiles := readInput("input.txt")

	board := make(grid)

	for _, tile := range tiles {

		length := len(tile)

		// reference point
		x := 0
		y := 0

		i := 0
		for i < length {
			switch a, b := tile[i:i+1], tile[i:min(i+2, length)]; {
			case a == "e":
				x++
				i++
			case a == "w":
				x--
				i++
			case b == "ne":
				// x++
				y--
				i += 2
			case b == "nw":
				x--
				y--
				i += 2
			case b == "se":
				x++
				y++
				i += 2
			case b == "sw":
				// x--
				y++
				i += 2
			}
		}
		pos := fmt.Sprintf("(%d)-(%d)", x, y)
		current := board[pos]
		board[pos] = !current
	}
	fmt.Println(board.countBlack())
	fmt.Println(board)
}

type grid map[string]bool

func (g grid) String() string {
	var minX, maxX, minY, maxY int
	var x, y int
	for k, _ := range g {
		fmt.Sscanf(k, "(%d)-(%d)", &x, &y)
		if x < minX {
			minX = x
		}
		if x > maxX {
			maxX = x
		}
		if y < minY {
			minY = y
		}
		if y > maxY {
			maxY = y
		}

	}

	var b strings.Builder
	for y := minY; y <= maxY; y++ {
		for x := minX; x <= maxX; x++ {
			pos := fmt.Sprintf("(%d)-(%d)", x, y)
			color := g[pos]
			if color {
				b.WriteString("#")
			} else {

				b.WriteString(".")
			}
		}
		b.WriteString("\n")
	}
	return b.String()
}

func (g grid) countBlack() int {
	total := 0
	for _, v := range g {
		if v {
			total++
		}
	}
	return total
}

func min(a, b int) int {
	if a < b {
		return a
	} else {
		return b
	}
}

func readInput(path string) []string {
	d, err := ioutil.ReadFile(path)
	if err != nil {
		log.Panic("could not read file: %v", err)
	}

	lines := strings.Split(strings.Trim(string(d), "\n"), "\n")

	return lines
}
