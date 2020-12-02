package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	entries, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	nvalid := 0
	for _, e := range entries {
		if e.Isvalid() {
			nvalid++
		}
	}
	fmt.Println(nvalid)
}

type entry struct {
	min      int
	max      int
	letter   string
	password string
}

func (e entry) Isvalid() bool {
	n := strings.Count(e.password, e.letter)
	return n >= e.min && n <= e.max
}

func readInput(path string) ([]entry, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("could not open %s: %v", path, err)
	}
	defer f.Close()

	var e []entry

	s := bufio.NewScanner(f)
	for s.Scan() {
		t := s.Text()
		var min, max int
		var letter, password string
		_, err := fmt.Sscanf(t, "%d-%d %1s: %s", &min, &max, &letter, &password)
		if err != nil {
			log.Fatal(fmt.Println("could not scan %s: %v", t, err))
		}
		e = append(e, entry{min, max, letter, password})

	}
	return e, s.Err()
}
