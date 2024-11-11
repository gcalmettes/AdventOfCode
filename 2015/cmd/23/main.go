package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Ins struct {
	Cmd      string
	Register string
	Value    int
}

func Run(instructions []Ins, targetRegister string, aRegisterValue int) int {
	registers := map[string]int{}
	registers["a"] = aRegisterValue
	cursor := 0
	for cursor < len(instructions) {
		ins := instructions[cursor]
		switch cmd := ins.Cmd; cmd {
		case "hlf":
			registers[ins.Register] /= 2
			cursor += 1
		case "tpl":
			registers[ins.Register] *= 3
			cursor += 1
		case "inc":
			registers[ins.Register] += 1
			cursor += 1
		case "jmp":
			cursor += ins.Value
		case "jie":
			if registers[ins.Register]%2 == 0 {
				cursor += ins.Value
			} else {
				cursor += 1
			}
		case "jio":
			if registers[ins.Register] == 1 {
				cursor += ins.Value
			} else {
				cursor += 1
			}
		}
	}
	return registers[targetRegister]
}

func ParseLine(s string) Ins {
	parts := strings.SplitN(s, " ", 2)
	cmd := parts[0]
	parts = strings.Split(parts[1], ", ")
	var value int
	var register string
	var err error
	if len(parts) == 1 {
		value, err = strconv.Atoi(parts[0])
		if err != nil {
			register = parts[0]
		}
	} else {
		register = parts[0]
		value, _ = strconv.Atoi(parts[1])
	}
	return Ins{cmd, register, value}
}

func main() {
	input, err := os.ReadFile("./inputs/23.txt")
	if err != nil {
		panic(err)
	}

	instructions := []Ins{}
	for _, line := range strings.Split(string(input), "\n") {
		if len(line) == 0 {
			continue
		}
		ins := ParseLine(line)
		instructions = append(instructions, ins)
	}

	p1 := Run(instructions, "b", 0)
	p2 := Run(instructions, "b", 1)

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
