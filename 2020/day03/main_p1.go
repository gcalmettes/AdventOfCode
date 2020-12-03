package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	field, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	stepX := 3
	stepY := 1
	x := 0 + stepX
	y := 0 + stepY

	hits := 0

	for {
		c, stop := field.Walk(x, y)
		if !stop && c == "#" {
			hits++
		}

		x += stepX
		y += stepY

		if y >= field.Size() {
			break
		}
		_ = stop
	}
	fmt.Println(hits)

}

type Field map[int]string

func (f Field) AddLine(i int, l string) {
	f[i] = l
}

func (f Field) Size() int {
	return len(f)
}

func (f Field) Walk(right, down int) (string, bool) {
	if down >= f.Size() {
		return "", true
	}
	line := f[down]

	c := string(line[right%len(line)])
	return c, false
}

func readInput(path string) (Field, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("could not open %s: %v", path, err)
	}
	defer f.Close()

	field := Field{}

	i := 0
	s := bufio.NewScanner(f)
	for s.Scan() {
		field.AddLine(i, s.Text())
		i++

	}
	return field, s.Err()
}
