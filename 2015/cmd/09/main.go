package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Node struct {
	Name string
}

type Edge struct {
	Node   *Node
	Weight int
}

type Nodes []Node

func (nodes *Nodes) ComputePath(graph map[Node][]*Edge, min bool) int {
	var total int

	if min {
		total = math.MaxInt
	}

	for _, start := range *nodes {
		var current Node
		otherNodes := Nodes{}
		for _, n := range *nodes {
			if n == start {
				current = n
			} else {
				otherNodes = append(otherNodes, n)
			}
		}

		visited := make(map[Node]bool)

		acc := 0
		idx := 0

		var threshold int

		for len(visited) < len(graph) {
			fmt.Printf("current: %s (%d)\n", current.Name, idx)

			visited[current] = true

			if min {
				threshold = math.MaxInt
			} else {
				threshold = 0
			}

			var next Node
			for _, e := range graph[current] {
				fmt.Printf("   analyzing edge: %s (%d)\n", e.Node.Name, e.Weight)
				if _, alreadyVisited := visited[*e.Node]; !alreadyVisited {
					fmt.Printf("   - %s not yet visited\n", e.Node.Name)
					if min && e.Weight < threshold {
						fmt.Printf("   - %s is current min\n", e.Node.Name)
						next = *e.Node
						threshold = e.Weight
					}
					if !min && e.Weight > threshold {
						fmt.Printf("   - %s is current max\n", e.Node.Name)
						next = *e.Node
						threshold = e.Weight
					}
				}
			}

			if next.Name == "" && len(visited) < len(graph) {
				// did not find a potentiel node to visit.
				// did we try all the possible start ?
				if idx == len(*nodes)-1 {
					panic("Did not find any solution")
				}
				// try another starting node
				idx += 1
				current = otherNodes[idx]
				// reset visited
				visited = make(map[Node]bool)
			} else {
				fmt.Printf(">> setting %s as current, adding %d to path\n", next.Name, threshold)
				visited[next] = true
				acc += threshold
				current = next
			}
			fmt.Printf("# current acc: %d\n", acc)
		}

		if len(visited) == len(graph) {
			if min && acc < total {
				total = acc
			}
			if !min && acc > total {
				total = acc
			}
		}
		acc = 0
	}
	return total
}

func main() {
	input, err := os.ReadFile("./inputs/09.txt")
	if err != nil {
		panic(err)
	}

	// construct graph
	graph := make(map[Node][]*Edge)
	for _, line := range strings.Split(string(input), "\n") {
		if line == "" {
			continue
		}
		parts := strings.SplitN(line, " = ", 2)
		weigth, _ := strconv.Atoi(parts[1])
		parts = strings.SplitN(parts[0], " to ", 2)
		d1 := parts[0]
		d2 := parts[1]
		n1 := Node{d1}
		n2 := Node{d2}
		e1 := &Edge{&n2, weigth}
		e2 := &Edge{&n1, weigth}
		graph[n1] = append(graph[n1], e1)
		graph[n2] = append(graph[n2], e2)
	}

	nodes := Nodes{}
	for n, _ := range graph {
		nodes = append(nodes, n)
	}

	p1 := nodes.ComputePath(graph, true)
	p2 := nodes.ComputePath(graph, false)

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
