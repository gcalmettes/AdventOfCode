package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

func main() {
	stacks := readInput("input.txt")

	p1 := stacks[0]
	p2 := stacks[1]

	// play game
	i := 1
	for {
		p1_card := p1[0]
		p2_card := p2[0]
		p1 = p1[1:]
		p2 = p2[1:]
		switch score := p1_card - p2_card; {
		case score > 0:
			p1 = append(p1, p1_card, p2_card)

		case score < 0:
			p2 = append(p2, p2_card, p1_card)
		}

		if len(p1) == 0 || len(p2) == 0 {
			// there is a winner
			break
		}

		i++
	}

	// get winner score
	score := 0
	switch l1, l2 := len(p1), len(p2); {
	case l1 != 0:
		for j := 1; j <= l1; j++ {
			score += j * p1[l1-j]
		}
	case l2 != 0:
		for j := 1; j <= l2; j++ {
			score += j * p2[l2-j]
		}
	}
	fmt.Println(score)

}

func readInput(path string) [][]int {
	d, err := ioutil.ReadFile(path)
	if err != nil {
		log.Fatal("could not read file: %v", err)
	}

	players := strings.Split(string(d), "\n\n")

	stacks := make([][]int, 0)
	for _, p := range players {
		stack := make([]int, 0)
		lines := strings.Split(strings.Trim(p, "\n"), "\n")
		for i, l := range lines {
			if i == 0 {
				// first line is player name
				continue
			}
			n, _ := strconv.Atoi(l)
			stack = append(stack, n)
			if i == len(lines)-1 {
				stacks = append(stacks, stack)
			}
		}
	}
	return stacks
}
