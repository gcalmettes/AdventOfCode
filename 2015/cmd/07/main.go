package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type (
	Operator string
	Wire     string
)

const (
	DIRECT Operator = "DIRECT"
	AND    Operator = "AND"
	OR     Operator = "OR"
	NOT    Operator = "NOT"
	LSHIFT Operator = "LSHIFT"
	RSHIFT Operator = "RSHIFT"
)

type Circuit map[Wire]int

func (c *Circuit) OkAll(wires []Wire) bool {
	all := true
	for _, w := range wires {
		if _, ok := (*c)[w]; !ok {
			all = false
		}
	}
	return all
}

type Instruction struct {
	Definition    string
	Operator      Operator
	OperatorValue int
	SrcWires      []Wire
	SrcValue      int
	To            Wire
}

func ParseInstruction(s string) *Instruction {
	parts := strings.SplitN(s, " -> ", 2)
	definition := strings.TrimSpace(parts[0])
	to := strings.TrimSpace(parts[1])

	found := false
	var srcValue int
	v, err := strconv.Atoi(definition)
	if err == nil {
		srcValue = v
		found = true
	}

	var op Operator
	var srcWires []Wire
	var opValue int
	for _, o := range []Operator{AND, OR, NOT, LSHIFT, RSHIFT} {
		if strings.Contains(definition, string(o)) {
			op = o
			wiresStr := strings.Replace(definition, string(o), "", 1)
			if (o == LSHIFT) || (o == RSHIFT) {
				// extract shift
				a := strings.Fields(wiresStr)
				srcWires = append(srcWires, Wire(a[0]))
				opValue, err = strconv.Atoi(a[1])

			} else {
				// extract all src wires
				for _, w := range strings.Fields(wiresStr) {

					srcVal, err := strconv.Atoi(w)
					if err != nil {
						srcWires = append(srcWires, Wire(w))
					} else {
						srcValue = srcVal
					}
				}
			}

			found = true
		}
	}

	if !found {
		// assignment of wire to another
		op = DIRECT
		srcWires = append(srcWires, Wire(definition))

	}

	return &Instruction{
		Definition:    definition,
		Operator:      op,
		OperatorValue: opValue,
		SrcWires:      srcWires,
		SrcValue:      srcValue,
		To:            Wire(to),
	}
}

func pop(insList *[]*Instruction) Instruction {
	f := len(*insList)
	rv := (*insList)[f-1]
	*insList = (*insList)[:f-1]
	return *rv
}

func computeIns(ins Instruction, circuit *Circuit) int {
	switch ins.Operator {
	case DIRECT:
		v, ok := (*circuit)[ins.SrcWires[0]]
		if !ok {
			panic("value not found")
		}
		return v
	case AND:
		var a, b int
		if len(ins.SrcWires) == 1 {
			v, ok := (*circuit)[ins.SrcWires[0]]
			if !ok {
				panic("value not found")
			}
			a = ins.SrcValue
			b = v
		} else {
			v1, ok := (*circuit)[ins.SrcWires[0]]
			v2, ok := (*circuit)[ins.SrcWires[1]]
			if !ok {
				panic("value not found")
			}
			a = v1
			b = v2
		}
		return a & b
	case OR:
		var a, b int
		if len(ins.SrcWires) == 1 {
			v, ok := (*circuit)[ins.SrcWires[0]]
			if !ok {
				panic("value not found")
			}
			a = ins.SrcValue
			b = v
		} else {
			v1, ok := (*circuit)[ins.SrcWires[0]]
			v2, ok := (*circuit)[ins.SrcWires[1]]
			if !ok {
				panic("value not found")
			}
			a = v1
			b = v2
		}
		return a | b
	case NOT:
		v, ok := (*circuit)[ins.SrcWires[0]]
		if !ok {
			panic("value not found")
		}
		notV := ^v
		if notV < 0 {
			// 16bits highest unsigned value 65535
			notV = 65536 + notV
		}
		return notV
	case LSHIFT:
		v, ok := (*circuit)[ins.SrcWires[0]]
		if !ok {
			panic("value not found")
		}
		return v << ins.OperatorValue
	case RSHIFT:
		v, ok := (*circuit)[ins.SrcWires[0]]
		if !ok {
			panic("value not found")
		}
		return v >> ins.OperatorValue
	}
	return 0
}

func RunCircuit(insPool []*Instruction) Circuit {
	var insTmp []*Instruction
	circuit := make(Circuit)

	for len(insPool) > 0 {
		ins := pop(&insPool)
		if len(ins.SrcWires) == 0 {
			// direct assignment
			circuit[ins.To] = ins.SrcValue
		} else if circuit.OkAll(ins.SrcWires) {
			// all the necessary src are in circuit, we can compute value
			newVal := computeIns(ins, &circuit)
			circuit[ins.To] = newVal
		} else {
			// cannot use yet instruction
			insTmp = append(insTmp, &ins)
		}
		if len(insPool) == 0 {
			if len(insTmp) > 0 {

				insPool = insTmp
				insTmp = nil
			} else {
				break
			}
		}
	}
	return circuit
}

func main() {
	input, err := os.ReadFile("./inputs/07.txt")
	if err != nil {
		panic(err)
	}
	var p1, p2 int

	var insPool []*Instruction
	for _, line := range strings.Split(string(input), "\n") {
		if line == "" {
			continue
		}
		ins := ParseInstruction(line)
		insPool = append(insPool, ins)

	}
	circuit := RunCircuit(insPool)
	p1, ok := circuit[Wire("a")]
	if !ok {
		panic("could not find a wire")
	}
	fmt.Printf("part1: %d\n", p1)

	// OVERRIDE value of b with p1 and reset all other values
	insPool = make([]*Instruction, 0)
	for _, line := range strings.Split(string(input), "\n") {
		if line == "" {
			continue
		}
		ins := ParseInstruction(line)
		if ins.To == Wire("b") {
			override := &Instruction{
				Operator: DIRECT,
				SrcValue: p1,
				To:       Wire("b"),
			}
			insPool = append(insPool, override)

		} else {
			insPool = append(insPool, ins)
		}

	}

	circuit = RunCircuit(insPool)

	p2, ok = circuit[Wire("a")]
	if !ok {
		panic("could not find a wire")
	}

	fmt.Printf("part2: %d\n", p2)
}
