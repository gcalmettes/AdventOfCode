package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strconv"
	"strings"
)

func main() {
	data, err := readInput("input.txt")
	if err != nil {
		fmt.Errorf("could not read input: %v", err)
	}

	sort.Ints(data)
	// add outlet
	data = append([]int{0}, data...)
	// add device
	device := data[len(data)-1] + 3
	data = append(data, device)

	// Make it a map for faster checking
	hash := map[int]bool{}
	for _, n := range data {
		hash[n] = true
	}

	// dynamic programming
	// Let's consider 0, 1, 2, 3
	// to go to 2
	max := data[len(data)-1]
	nPaths := make([]int, max)
	nPaths[0] = 1 // only one way to go to one

	if hash[1] {

		nPaths[1] = 1 // 0-1
	}

	if hash[2] && hash[1] {

		nPaths[2] = 2 // 0-1-2 || 0-2
	} else {

		nPaths[2] = 1 // only 0-2 if 1 not present
	}

	for i := 3; i < max; i++ {
		if !hash[i] {
			continue
		}

		nPaths[i] = nPaths[i-3] + nPaths[i-2] + nPaths[i-1]
	}

	fmt.Println(nPaths[max-3])

}

func readInput(path string) ([]int, error) {
	f, err := ioutil.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("could not open %s: %v", path, err)
	}

	d := make([]int, 0)
	for _, n := range strings.Split(strings.TrimSpace(string(f)), "\n") {
		i, _ := strconv.Atoi(n)
		d = append(d, i)
	}

	return d, nil
}
