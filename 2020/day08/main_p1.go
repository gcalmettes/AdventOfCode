package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	ins, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	p := program{ins, 0, 0}
	for p.next() {
		continue
	}
	fmt.Println(p.score)
}

type step struct {
	kind     string
	value    int
	executed bool
}
type program struct {
	ins     []*step
	current int
	score   int
}

func (s *step) done() {
	s.executed = true
}
func (p *program) next() bool {
	s := p.ins[p.current]
	if s.executed {
		fmt.Println("It's a loop!")
		return false
	}
	p.ins[p.current].done()
	switch s.kind {
	case "nop":
		p.current++
	case "acc":
		p.current++
		p.score += s.value
	case "jmp":
		p.current += s.value
	}
	return true
}

// func move
func readInput(path string) ([]*step, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("could not open %s: %v", path, err)
	}
	defer f.Close()
	d := make([]*step, 0)
	s := bufio.NewScanner(f)
	for s.Scan() {
		var k string
		var v int
		fmt.Sscanf(s.Text(), "%s %d", &k, &v)
		d = append(d, &step{k, v, false})
	}
	return d, s.Err()
}
