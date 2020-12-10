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

	var ones, threes int
	others := make([]int, 0)
	for i := 1; i < len(data); i++ {
		diff := data[i] - data[i-1]
		if diff == 1 {
			ones++
		} else if diff == 3 {
			threes++
		} else {
			others = append(others, diff)
		}
	}
	if len(others) > 0 {
		fmt.Printf("unknown differences were found: %v\n", others)
	}
	fmt.Println(ones * threes)
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
