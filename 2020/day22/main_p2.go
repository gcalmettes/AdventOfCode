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

	_, winnerStack := playGame(p1, p2)
	l := len(winnerStack)

	score := 0
	for i := 1; i <= l; i++ {
		score += i * winnerStack[l-i]
	}
	fmt.Println(score)

}

func playGame(stack_p1, stack_p2 []int) (int, []int) {
	var winningPlayer int
	var winningStack []int

	// copy stack to ensure we don't change original slices
	stack1 := append([]int{}, stack_p1...)
	stack2 := append([]int{}, stack_p2...)

	seen := make(map[string]bool)

	for {
		// Rule 1:
		// did we already see those exact stacks ?
		combination := fmt.Sprint("%v-%v", stack1, stack2)
		alreadySeen := seen[combination]
		if alreadySeen {
			// if alreadySeen1 || alreadySeen2 {
			winningPlayer = 1
			winningStack = stack1
			// fmt.Println("Winner by infinite loop.")
			break
		} else {
			seen[combination] = true
		}

		p1_card := stack1[0]
		p2_card := stack2[0]

		stack1 = stack1[1:]
		stack2 = stack2[1:]

		// Rule 2:
		// do the players have at least as many card left as the card
		// they dealt ?
		if len(stack1) >= p1_card && len(stack2) >= p2_card {
			// Sub game to determine winner of that round
			w, _ := playGame(stack1[:p1_card], stack2[:p2_card])
			switch w {
			case 1:
				stack1 = append(stack1, p1_card, p2_card)

			case 2:
				stack2 = append(stack2, p2_card, p1_card)
			}
		} else {
			// Rule 3:
			// highest value card wins
			switch score := p1_card - p2_card; {
			case score > 0:
				stack1 = append(stack1, p1_card, p2_card)

			case score < 0:
				stack2 = append(stack2, p2_card, p1_card)
			}

		}

		if len(stack1) == 0 || len(stack2) == 0 {
			// there is a winner
			break
		}
	}

	if len(stack1) > 0 {
		winningPlayer = 1
		winningStack = stack1
	} else if len(stack2) > 0 {
		winningPlayer = 2
		winningStack = stack2
	}
	return winningPlayer, winningStack
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
