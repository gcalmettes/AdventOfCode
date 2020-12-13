package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
)

func main() {
	ins, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	var x, y int
	dx, dy := 10, 1
	for _, s := range ins {
		if s.dir == "N" {
			dy += s.value
		}
		if s.dir == "S" {
			dy -= s.value
		}
		if s.dir == "E" {
			dx += s.value
		}
		if s.dir == "W" {
			dx -= s.value
		}
		if s.dir == "L" {
			f := int(math.Cos(float64(s.value) * math.Pi / 180.0))
			g := int(math.Sin(float64(s.value) * math.Pi / 180.0))
			dx, dy = f*dx-g*dy, g*dx+f*dy
		}
		if s.dir == "R" {
			f := int(math.Cos(-float64(s.value) * math.Pi / 180.0))
			g := int(math.Sin(-float64(s.value) * math.Pi / 180.0))
			dx, dy = f*dx-g*dy, g*dx+f*dy
		}
		if s.dir == "F" {
			x += dx * s.value
			y += dy * s.value
		}
	}
	fmt.Println(math.Abs(float64(x)) + math.Abs(float64(y)))
}

type step struct {
	dir   string
	value int
}

func readInput(path string) ([]step, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("could not open %s: %v", path, err)
	}
	defer f.Close()

	var data []step

	s := bufio.NewScanner(f)
	for s.Scan() {
		t := s.Text()
		var v int
		var dir string
		_, err := fmt.Sscanf(t, "%1s%d", &dir, &v)
		if err != nil {
			log.Fatal(fmt.Println("could not scan %s: %v", t, err))
		}
		data = append(data, step{dir, v})

	}
	return data, s.Err()
}
