package main

import (
	"fmt"
	"strings"
)

func main() {

	containers := []int{11, 30, 47, 31, 32, 36, 3, 1, 5, 3, 32, 36, 15, 11, 46, 26, 28, 1, 19, 3}

	combinations := []string{}
	for _ = range len(containers) {
		if len(combinations) == 0 {
			combinations = append(combinations, "0")
			combinations = append(combinations, "1")
		} else {
			toAdd := []string{}
			for i := range len(combinations) {
				toAdd = append(toAdd, combinations[i]+"1")
				combinations[i] = combinations[i] + "0"
			}
			combinations = append(combinations, toAdd...)
		}
	}

	var p1, p2 int

	valid := make(map[int]int)

	for _, c := range combinations {
		score := 0
		for i, q := range c {
			if q == '1' {
				score += containers[i]
			}
			if score > 150 {
				break
			}
		}
		if score == 150 {
			p1 += 1
			c = strings.ReplaceAll(c, "0", "")
			valid[len(c)] += 1
		}
	}

	max := len(containers)
	for k, v := range valid {
		if k < max {
			max = k
			p2 = v
		}
	}

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
