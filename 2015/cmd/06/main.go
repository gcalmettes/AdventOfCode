package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Action string

const (
	TurnOn  Action = "turn on"
	TurnOff Action = "turn off"
	Toggle  Action = "toggle"
)

type Step struct {
	Action Action
	x0     int
	x1     int
	y0     int
	y1     int
}

func ParseStep(s string) *Step {
	parts := strings.SplitN(s, " through ", 2)

	var action Action
	start := strings.Split(parts[0], " ")
	switch len(start) {
	case 2:
		action = Toggle
	case 3:
		if start[1] == "on" {
			action = TurnOn
		} else {
			action = TurnOff
		}
	default:
		fmt.Println(start)
		panic("should not happen")

	}
	start = strings.Split(start[len(start)-1], ",")
	end := strings.SplitN(parts[1], ",", 2)

	x0, err := strconv.Atoi(start[0])
	y0, err := strconv.Atoi(start[1])
	x1, err := strconv.Atoi(end[0])
	y1, err := strconv.Atoi(end[1])
	if err != nil {
		panic(err)
	}

	return &Step{
		action, x0, x1, y0, y1,
	}
}

type Pos struct {
	x int
	y int
}

type Light struct {
	On    bool
	Value int
}

func main() {
	input, err := os.ReadFile("./inputs/06.txt")
	if err != nil {
		panic(err)
	}

	grid := make(map[Pos]bool)
	grid2 := make(map[Pos]int)

	for _, s := range strings.Split(string(input), "\n") {
		if s == "" {
			continue
		}

		step := ParseStep(s)
		for i := range step.x1 - step.x0 + 1 {
			for j := range step.y1 - step.y0 + 1 {
				switch step.Action {
				case TurnOn:
					grid[Pos{step.x0 + i, step.y0 + j}] = true
					grid2[Pos{step.x0 + i, step.y0 + j}] += 1
				case TurnOff:
					grid[Pos{step.x0 + i, step.y0 + j}] = false
					grid2[Pos{step.x0 + i, step.y0 + j}] -= 1
					if grid2[Pos{step.x0 + i, step.y0 + j}] < 0 {
						grid2[Pos{step.x0 + i, step.y0 + j}] = 0
					}
				case Toggle:
					grid[Pos{step.x0 + i, step.y0 + j}] = !grid[Pos{step.x0 + i, step.y0 + j}]
					grid2[Pos{step.x0 + i, step.y0 + j}] += 2
				}
			}
		}
	}

	p1 := 0
	for _, v := range grid {
		if v {
			p1++
		}
	}
	p2 := 0
	for _, v := range grid2 {
		p2 += v
	}

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part1: %d\n", p2)
}
