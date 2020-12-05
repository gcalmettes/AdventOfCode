package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	data, err := readInput("input.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	highest := 0
	for _, d := range data {
		s, err := parseSeat(d)
		if err != nil {
			fmt.Println("could not make seat from boarding pass %v", d)
		}

		pid := s.getPid()
		if pid > highest {
			highest = pid
		}
	}
	fmt.Println(highest)

}

type Seat struct {
	row int
	col int
}

func (s Seat) getPid() int {
	return s.row*8 + s.col
}

func parseSeat(boardingPass string) (Seat, error) {
	hash := map[string]string{
		"F": "0",
		"B": "1",
		"R": "1",
		"L": "0",
	}

	bp := []byte(boardingPass)
	for i, b := range bp {
		bp[i] = []byte(hash[string(b)])[0]
	}

	row, err := strconv.ParseInt(string(bp[:7]), 2, 64)
	if err != nil {
		return Seat{}, errors.New("could not parse int64")
	}
	col, err := strconv.ParseInt(string(bp[7:]), 2, 64)
	if err != nil {
		return Seat{}, errors.New("could not parse int64")
	}

	return Seat{int(row), int(col)}, nil
}

func readInput(path string) ([]string, error) {
	f, err := ioutil.ReadFile(path)
	if err != nil {
		return []string{}, fmt.Errorf("could not open %s: %v", path, err)
	}

	seats := strings.Split(strings.TrimSpace(string(f)), "\n")

	return seats, nil
}
