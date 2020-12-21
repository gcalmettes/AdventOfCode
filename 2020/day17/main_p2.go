package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
)

func main() {

	config, _ := ioutil.ReadFile("input.txt")
	// config, _ := ioutil.ReadFile("input_test.txt")

	lines := strings.Split(strings.Trim(string(config), "\n"), "\n")

	state := make(map[cube]bool)

	for i, l := range lines {
		for j, c := range l {
			isActive := string(c) == "#"
			state[cube{j, i, 0, 0}] = isActive
		}
	}

	for t := 0; t < 6; t++ {

		currentState := make(map[cube]bool)
		for k, v := range state {
			currentState[k] = v
		}
		xmin, xmax, ymin, ymax, zmin, zmax, wmin, wmax := getBoundaries(state)
		for x := xmin - 1; x <= xmax+1; x++ {
			for y := ymin - 1; y <= ymax+1; y++ {
				for z := zmin - 1; z <= zmax+1; z++ {
					for w := wmin - 1; w <= wmax+1; w++ {
						c := cube{x, y, z, w}
						activeNeighbors := c.countActiveNeighbors(currentState)

						isActive, ok := currentState[c]
						if (ok && isActive) && (activeNeighbors != 2 || activeNeighbors != 3) {
							if (activeNeighbors == 2) || (activeNeighbors == 3) {
								// active and 2 or 3 active neighbors => stay active
								state[c] = true

							} else {
								// becomes inactive
								state[c] = false
							}
						} else if (!ok || !isActive) && activeNeighbors == 3 {
							// inactive and 3 neighbors active => becomes active
							state[c] = true

						} else {
							state[c] = false
						}
					}
				}
			}
		}
	}

	total := 0
	for _, v := range state {
		if v {
			total++
		}
	}
	fmt.Println(total)

}

type cube struct {
	x, y, z, w int
}

func (c cube) getNeighbors() []cube {
	neighbors := []cube{}
	for i := -1; i <= 1; i++ {
		for j := -1; j <= 1; j++ {
			for k := -1; k <= 1; k++ {
				for l := -1; l <= 1; l++ {
					if i == 0 && j == 0 && k == 0 && l == 0 {
						continue
					}
					neighbors = append(neighbors, cube{c.x + i, c.y + j, c.z + k, c.w + l})
				}
			}
		}
	}
	return neighbors
}

func (c cube) countActiveNeighbors(state map[cube]bool) int {
	neighbors := c.getNeighbors()
	count := 0
	for _, n := range neighbors {
		if c, ok := state[n]; ok && c {
			count++
		}

	}
	return count
}

func getBoundaries(s map[cube]bool) (xmin, xmax, ymin, ymax, zmin, zmax, wmin, wmax int) {
	xs := []int{}
	ys := []int{}
	zs := []int{}
	ws := []int{}

	for c, _ := range s {
		xs = append(xs, c.x)
		ys = append(ys, c.y)
		zs = append(zs, c.z)
		ws = append(ws, c.w)
	}

	sort.Ints(xs)
	sort.Ints(ys)
	sort.Ints(zs)
	sort.Ints(ws)
	return xs[0], xs[len(xs)-1], ys[0], ys[len(ys)-1], zs[0], zs[len(zs)-1], ws[0], ws[len(ws)-1]
}
