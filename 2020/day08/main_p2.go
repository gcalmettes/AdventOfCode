package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	p, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	for i := 0; i < len(p); i++ {
		ins := p[i]
		switch ins.kind {
		case "nop":
			p[i].kind = "jmp"
		case "jmp":
			p[i].kind = "nop"
		}
		score, ok := run(p)
		p[i] = ins
		if ok {
			fmt.Println(score)
		}
	}

}

type step struct {
	kind  string
	value int
}

type program []step

func run(p program) (int, bool) {

	var current, score int
	seen := make(map[int]bool)

	for current < len(p) {
		if seen[current] {
			return score, false
		}
		seen[current] = true
		switch p[current].kind {
		case "acc":
			score += p[current].value
		case "jmp":
			current += p[current].value - 1
		}
		current++

	}

	return score, true
}

func readInput(path string) (program, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("could not open %s: %v", path, err)
	}
	defer f.Close()
	d := make([]step, 0)
	s := bufio.NewScanner(f)
	for s.Scan() {
		var k string
		var v int
		fmt.Sscanf(s.Text(), "%s %d", &k, &v)
		d = append(d, step{k, v})
	}
	return d, s.Err()
}
