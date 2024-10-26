package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Box struct {
	Length int64
	Width  int64
	Height int64
}

func (b *Box) Area() int64 {
	s1 := b.Length * b.Width
	s2 := b.Width * b.Height
	s3 := b.Height * b.Length

	extra := s1
	for _, t := range []int64{s2, s3} {
		if t < extra {
			extra = t
		}
	}
	return extra + 2*s1 + 2*s2 + 2*s3
}

func (b *Box) Ribbon() int {
	sizes := []int{int(b.Length), int(b.Width), int(b.Height)}
	sort.Ints(sizes)
	r1 := sizes[0]
	r2 := sizes[1]

	return 2*r1 + 2*r2 + (r1 * r2 * sizes[2])
}

func NewBox(s string) *Box {
	parts := strings.SplitN(s, "x", 3)
	l, err := strconv.ParseInt(parts[0], 10, 64)
	w, err := strconv.ParseInt(parts[1], 10, 64)
	h, err := strconv.ParseInt(parts[2], 10, 64)
	if err != nil {
		panic(err)
	}
	return &Box{l, w, h}
}

func main() {
	input, err := os.ReadFile("./inputs/02.txt")
	if err != nil {
		panic(err)
	}
	var totalArea int64
	var totalRibbon int
	for _, s := range strings.Split(string(input), "\n") {
		if s == "" {
			continue
		}
		box := NewBox(s)
		totalArea += box.Area()
		totalRibbon += box.Ribbon()
	}
	fmt.Printf("part1: %d\n", totalArea)
	fmt.Printf("part2: %d\n", totalRibbon)
}
