package main

import (
	"fmt"
	"strconv"
	"strings"
)

func GetNextSeq(s string) string {
	var seq strings.Builder
	var current rune
	var count int

	for _, c := range s {
		if c != current {
			if count > 0 {
				seq.WriteString(strconv.Itoa(count))
				seq.WriteString(string(current))
			}

			current = c
			count = 1
		} else {
			count++
		}
	}

	seq.WriteString(strconv.Itoa(count))
	seq.WriteString(string(current))
	return seq.String()
}

const INPUT string = "1113122113"

func main() {
	seq := INPUT

	var p1, p2 int

	for i := range 50 {
		seq = GetNextSeq(seq)
		if i == 39 {
			p1 = len(seq)
		}
	}
	p2 = len(seq)

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
