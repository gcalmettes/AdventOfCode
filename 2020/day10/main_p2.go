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
	// we cannot go further than 3 steps, so if a number is present in the adpators, the numbers
	// of ways to get to it is the sum of the numbers of ways to go to the three preceeding numbers.
	// ex: if 34-35-36-37 are presents, then to go to 37 we come from either 34-37, 35-37 or 36-37,
	// meaning that we have to add all the ways to go to 34, 35 and 36 to know the numbers of ways
	// to go to 37.
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
