package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
	"time"
)

func main() {
	data, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	width := len(data[0])
	height := len(data)
	g := grid{width, height, map[xy]string{}}

	for y, line := range data {
		for x, status := range line {
			g.cells[xy{x, y}] = string(status)
		}
	}

	i := 0
	for {
		// puth cursor at the top left
		// see https://student.cs.uwaterloo.ca/~cs452/terminal.html
		fmt.Print("\033[H\033[2J")
		fmt.Println(g)
		before := g.String()
		g.step()
		after := g.String()
		if before == after {
			break
		}
		i++
		time.Sleep(200 * time.Millisecond)
	}
}

type xy struct {
	x, y int
}

// type grid map[xy]string
type grid struct {
	width  int
	height int
	cells  map[xy]string
}

func (g grid) String() string {
	var n int
	var out strings.Builder
	for y := 0; y < g.height; y++ {
		for x := 0; x < g.width; x++ {
			status := g.cells[xy{x, y}]
			if status == "#" {
				n++
				fmt.Fprintf(&out, "\033[33m")
				fmt.Fprintf(&out, status)
				fmt.Fprintf(&out, "\033[0m")
			} else {

				fmt.Fprintf(&out, status)
			}

		}
		fmt.Fprint(&out, "\n")
	}
	// Red color
	fmt.Fprintf(&out, "\033[31m")
	fmt.Fprintf(&out, "Occupied: \033[30m\033[47m%d\033[49m\n\n", n)
	// Reset
	fmt.Fprintf(&out, "\033[0m")
	return out.String()

}

func (g grid) step() {
	// we must freeze current state
	var current = make(map[xy]string)
	for k, v := range g.cells {
		current[k] = v
	}
	for p, s := range current {
		if s == "." {
			// not a seat, pass
			continue
		}
		o := p.occupiedAround(grid{g.width, g.height, current})
		if (s == "L") && (o == 0) {
			// becomes occupied
			g.cells[p] = "#"
		} else if (s == "#") && (o >= 4) {
			// becomes empty
			g.cells[p] = "L"
		}
	}
}

func (pos xy) neighbors(w, h int) []xy {
	n := make([]xy, 0)

	addLeft := pos.x-1 >= 0
	addRight := pos.x+1 < w
	addUpper := pos.y-1 >= 0
	addLower := pos.y+1 < h

	if addLeft {
		n = append(n, xy{pos.x - 1, pos.y})
		if addUpper {
			n = append(n, xy{pos.x - 1, pos.y - 1})
		}
		if addLower {
			n = append(n, xy{pos.x - 1, pos.y + 1})
		}
	}
	if addRight {
		n = append(n, xy{pos.x + 1, pos.y})
		if addUpper {
			n = append(n, xy{pos.x + 1, pos.y - 1})
		}
		if addLower {
			n = append(n, xy{pos.x + 1, pos.y + 1})
		}
	}
	if addUpper {
		n = append(n, xy{pos.x, pos.y - 1})
	}
	if addLower {
		n = append(n, xy{pos.x, pos.y + 1})
	}
	return n
}

func (pos xy) occupiedAround(g grid) int {

	neighbors := pos.neighbors(g.width, g.height)
	c := 0
	for _, n := range neighbors {
		if g.cells[n] == "#" {
			c++
		}
	}
	return c
}

func readInput(path string) ([]string, error) {
	d, err := ioutil.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("could not open %s: %v", path, err)
	}

	lines := strings.Split(string(d), "\n")

	return lines, nil
}
