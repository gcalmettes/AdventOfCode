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
			}
			fmt.Fprintf(&out, status)
		}
		fmt.Fprint(&out, "\n")
	}
	fmt.Fprintf(&out, "Occupied: %d\n", n)
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
		} else if (s == "#") && (o >= 5) {
			// becomes empty
			g.cells[p] = "L"
		}
	}
}

func (pos xy) visibleNeighbors(g grid) []xy {
	w := g.width
	h := g.height

	n := make([]xy, 0)

	// left
	i := 1
	for pos.x-i >= 0 {
		if g.cells[xy{pos.x - i, pos.y}] != "." {
			n = append(n, xy{pos.x - i, pos.y})
			break
		}
		i++
	}
	// right
	i = 1
	for pos.x+i < w {
		if g.cells[xy{pos.x + i, pos.y}] != "." {
			n = append(n, xy{pos.x + i, pos.y})
			break
		}
		i++
	}
	// top
	i = 1
	for pos.y-i >= 0 {
		if g.cells[xy{pos.x, pos.y - i}] != "." {
			n = append(n, xy{pos.x, pos.y - i})
			break
		}
		i++
	}
	// bottom
	i = 1
	for pos.y+i < h {
		if g.cells[xy{pos.x, pos.y + i}] != "." {
			n = append(n, xy{pos.x, pos.y + i})
			break
		}
		i++
	}
	// top-left
	i = 1
	for pos.y-i >= 0 && pos.x-i >= 0 {
		if g.cells[xy{pos.x - i, pos.y - i}] != "." {
			n = append(n, xy{pos.x - i, pos.y - i})
			break
		}
		i++
	}
	// top-right
	i = 1
	for pos.y-i >= 0 && pos.x+i < w {
		if g.cells[xy{pos.x + i, pos.y - i}] != "." {
			n = append(n, xy{pos.x + i, pos.y - i})
			break
		}
		i++
	}
	// bottom-left
	i = 1
	for pos.y+i < h && pos.x-i >= 0 {
		if g.cells[xy{pos.x - i, pos.y + i}] != "." {
			n = append(n, xy{pos.x - i, pos.y + i})
			break
		}
		i++
	}
	// bottom-right
	i = 1
	for pos.y+i < h && pos.x+i < w {
		if g.cells[xy{pos.x + i, pos.y + i}] != "." {
			n = append(n, xy{pos.x + i, pos.y + i})
			break
		}
		i++
	}

	return n
}

func (pos xy) occupiedAround(g grid) int {

	neighbors := pos.visibleNeighbors(g)
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
