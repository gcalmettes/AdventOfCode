package main

import (
	"fmt"
	"os"
	"strings"
)

type santaString string

func (s santaString) IsNicePart1() bool {
	vowelsCheck := false
	doubleCheck := false
	notContainCheck := true

	var vowelsCount int
	for _, c := range s {
		switch c {
		case 'a', 'e', 'i', 'o', 'u':
			vowelsCount++
		}
		if vowelsCount >= 3 {
			vowelsCheck = true

			break
		}
	}

	for i, c := range s {
		if c == rune(string(s)[i+1]) {
			doubleCheck = true
			break
		}
		if i == len(s)-2 {
			break
		}
	}

	for _, pattern := range []string{"ab", "cd", "pq", "xy"} {
		if strings.Contains(string(s), pattern) {
			notContainCheck = false
			break
		}
	}

	return vowelsCheck && doubleCheck && notContainCheck
}

func (s santaString) IsNicePart2() bool {
	repeatDoubleCheck := false
	repeatSingleCheck := false

	i := 0
	for i < len(s)-1 {
		if strings.Count(string(s), string(s)[i:i+2]) > 1 {
			repeatDoubleCheck = true
			break
		}
		i++
	}

	i = 0
	for i < len(s)-2 {
		if string(s)[i] == string(s)[i+2] {
			repeatSingleCheck = true
			break
		}
		i++
	}
	// fmt.Println(repeatDoubleCheck, repeatSingleCheck)
	return repeatDoubleCheck && repeatSingleCheck
}

func main() {
	input, err := os.ReadFile("./inputs/05.txt")
	if err != nil {
		panic(err)
	}

	var p1 int
	var p2 int

	for _, s := range strings.Split(string(input), "\n") {
		if s == "" {
			continue
		}
		if santaString(s).IsNicePart1() {
			p1++
		}
		if santaString(s).IsNicePart2() {
			p2++
		}
	}

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part1: %d\n", p2)
}
