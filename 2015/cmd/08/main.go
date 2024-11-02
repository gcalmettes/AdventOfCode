package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Literal string

func (l *Literal) CodeCharactersLen() int {
	return len(*l)
}

func (l *Literal) InMemoryCharactersLen() int {
	im := string(*l)
	hexa := regexp.MustCompile(`\\x[0-9a-fA-F]{2}`)
	im = hexa.ReplaceAllString(im, "a")
	im = strings.ReplaceAll(im, "\\\"", "\"")
	im = strings.ReplaceAll(im, "\\\\", "\\")
	return len(im) - 2
}

func (l *Literal) EncodedLen() int {
	im := string(*l)
	im = strconv.Quote(im)
	return len(im)
}
func main() {
	input, err := os.ReadFile("./inputs/08.txt")
	if err != nil {
		panic(err)
	}

	var p1, p2 int

	for _, line := range strings.Split(string(input), "\n") {
		if line == "" {
			continue
		}
		literal := Literal(line)
		p1 += (literal.CodeCharactersLen() - literal.InMemoryCharactersLen())
		p2 += (literal.EncodedLen() - literal.CodeCharactersLen())
	}
	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
