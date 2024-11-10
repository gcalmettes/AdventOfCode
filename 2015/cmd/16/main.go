package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Aunt map[string]int

func (a Aunt) IsMatch(withRange bool) bool {
	characteristics := map[string]int{
		"children":    3,
		"cats":        7,
		"samoyeds":    2,
		"pomeranians": 3,
		"akitas":      0,
		"vizslas":     0,
		"goldfish":    5,
		"trees":       3,
		"cars":        2,
		"perfumes":    1,
	}

	for k, v := range characteristics {
		if !withRange {
			if char, ok := a[k]; ok && char != v {
				return false
			}
		} else {
			char, ok := a[k]
			if ok {
				if k == "cats" || k == "trees" {
					if char <= v {
						return false
					}
				} else if k == "pomeranians" || k == "goldfish" {
					if char >= v {
						return false
					}
				} else {
					if char != v {
						return false

					}
				}
			}
		}
	}
	return true
}

func ParseLine(s string) Aunt {
	aunt := Aunt{}
	split := strings.SplitN(s, ": ", 2)
	attrs := strings.Split(split[1], ", ")
	for _, attr := range attrs {
		kv := strings.Split(attr, ": ")
		val, _ := strconv.Atoi(kv[1])
		aunt[kv[0]] = val
	}
	return aunt
}

func main() {
	input, err := os.ReadFile("./inputs/16.txt")
	if err != nil {
		panic(err)
	}

	var p1, p2 int

	i := 1
	for _, line := range strings.Split(string(input), "\n") {
		if len(line) == 0 {
			continue
		}
		aunt := ParseLine(line)
		if aunt.IsMatch(false) {
			p1 = i
		}
		if aunt.IsMatch(true) {
			p2 = i
		}
		i++
	}

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
