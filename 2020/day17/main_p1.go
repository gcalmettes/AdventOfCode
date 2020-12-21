package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
)

func main() {

	config, _ := ioutil.ReadFile("input.txt")

	lines := strings.Split(strings.Trim(string(config), "\n"), "\n")

	state := make(map[cube]bool)

	for i, l := range lines {
		for j, c := range l {
			isActive := string(c) == "#"
			state[cube{j, i, 0}] = isActive
		}
	}

	for t := 0; t < 6; t++ {

		currentState := make(map[cube]bool)
		for k, v := range state {
			currentState[k] = v
		}
		xmin, xmax, ymin, ymax, zmin, zmax := getBoundaries(state)
		for x := xmin - 1; x <= xmax+1; x++ {
			for y := ymin - 1; y <= ymax+1; y++ {
				for z := zmin - 1; z <= zmax+1; z++ {
					c := cube{x, y, z}
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

	total := 0
	for _, v := range state {
		if v {
			total++
		}
	}
	fmt.Println(total)

}

func printSlice(state map[cube]bool, depth int) {
	xs := make(map[int]bool)
	ys := make(map[int]bool)

	for k, _ := range state {
		if k.z == depth {
			xs[k.x] = true
			ys[k.y] = true
		}
	}

	xl := make([]int, len(xs))
	yl := make([]int, len(ys))
	i := 0
	for k := range xs {
		xl[i] = k
		i++
	}
	i = 0
	for k := range ys {
		yl[i] = k
		i++
	}

	sort.Ints(xl)
	sort.Ints(yl)

	for _, y := range yl {
		for _, x := range xl {
			switch state[cube{x, y, depth}] {
			case true:
				fmt.Print("#")
			case false:
				fmt.Print(".")
			}
		}
		fmt.Print("\n")
	}
	fmt.Println("\n")
}

type cube struct {
	x, y, z int
}

func (c cube) getNeighbors() []cube {
	neighbors := []cube{}
	for i := -1; i <= 1; i++ {
		for j := -1; j <= 1; j++ {
			for k := -1; k <= 1; k++ {
				if i == 0 && j == 0 && k == 0 {
					continue
				}
				neighbors = append(neighbors, cube{c.x + i, c.y + j, c.z + k})
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

func getBoundaries(s map[cube]bool) (xmin, xmax, ymin, ymax, zmin, zmax int) {
	xs := []int{}
	ys := []int{}
	zs := []int{}

	for c, _ := range s {
		xs = append(xs, c.x)
		ys = append(ys, c.y)
		zs = append(zs, c.z)
	}

	sort.Ints(xs)
	sort.Ints(ys)
	sort.Ints(zs)
	return xs[0], xs[len(xs)-1], ys[0], ys[len(ys)-1], zs[0], zs[len(zs)-1]
}
