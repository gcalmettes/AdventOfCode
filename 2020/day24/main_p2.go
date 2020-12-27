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

	// Initial setup
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
				y++
				i += 2
			}
		}
		pos := fmt.Sprintf("(%d)-(%d)", x, y)
		current := board[pos]
		board[pos] = !current
	}

	day := 0
	for day < 100 {
		day++

		// make grid with current tiles and neighbors
		tmp := make(grid)
		for pos, v := range board {
			tmp[pos] = v
			neighbors := getNeighbors(pos)
			for _, p := range neighbors {
				b := board[p]
				tmp[p] = b
			}

		}
		// flip needed tiles
		for pos := range tmp {
			neighbors := getNeighbors(pos)
			c := 0
			for _, p := range neighbors {
				if board[p] {
					c++
				}
			}
			switch isBlack := board[pos]; {
			case isBlack && (c == 0 || c > 2):
				tmp[pos] = !isBlack
			case !isBlack && c == 2:
				tmp[pos] = !isBlack
			default:
				tmp[pos] = isBlack
			}

		}
		board = tmp
		// fmt.Print("\033[H\033[2J")
		// fmt.Println(board)
	}
	fmt.Println(board.countBlack())
}

type grid map[string]bool

func (g grid) getBounds() (int, int, int, int) {
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
	return minX, maxX, minY, maxY
}

func (g grid) String() string {
	minX, maxX, minY, maxY := g.getBounds()
	// minX, maxX, minY, maxY := -50, 50, -50, 50

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

func getNeighbors(pos string) []string {

	var x, y int
	fmt.Sscanf(pos, "(%d)-(%d)", &x, &y)
	return []string{
		fmt.Sprintf("(%d)-(%d)", x+1, y),
		fmt.Sprintf("(%d)-(%d)", x-1, y),
		fmt.Sprintf("(%d)-(%d)", x, y+1),
		fmt.Sprintf("(%d)-(%d)", x, y-1),
		fmt.Sprintf("(%d)-(%d)", x-1, y-1),
		fmt.Sprintf("(%d)-(%d)", x+1, y+1),
	}
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
