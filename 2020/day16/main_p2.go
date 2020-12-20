package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

func main() {
	input := readInput("input.txt")
	rules := input[0]
	ticket := input[1]
	nearby := input[2]

	// Authorized values for fields
	authorized := make(map[int]bool)
	rulesAuthorizedInt := make(map[string]map[int]bool)

	for _, line := range strings.Split(rules, "\n") {
		parts := strings.Split(line, ": ")

		values := make(map[int]bool)

		ranges := strings.Split(parts[1], " or ")
		for _, r := range ranges {
			bounds := strings.Split(r, "-")
			lower, _ := strconv.Atoi(bounds[0])
			upper, _ := strconv.Atoi(bounds[1])

			for i := lower; i <= upper; i++ {
				authorized[i] = true
				values[i] = true
			}
		}
		rulesAuthorizedInt[parts[0]] = values
	}

	validTickets := [][]int{}

	for _, t := range strings.Split(nearby, "\n") {
		values := strings.Split(t, ",")
		valuesInt := []int{}
		valid := true
		for _, v := range values {
			n, _ := strconv.Atoi(v)
			valuesInt = append(valuesInt, n)
			if !authorized[n] {
				valid = false
			}
		}
		if valid {
			validTickets = append(validTickets, valuesInt)
		}
	}

	positions := make(map[int][]int)
	for _, values := range validTickets {
		for p, v := range values {
			positions[p] = append(positions[p], v)
		}
	}

	matchPositions := make(map[int][]string)

	for pos, values := range positions {
		for s, autorizedValues := range rulesAuthorizedInt {
			all := true
			for _, v := range values {
				if !autorizedValues[v] {
					all = false
				}
			}
			if all {
				matchPositions[pos] = append(matchPositions[pos], s)
			}

		}
	}

	matches := make(map[string]int)
	myTicket := []int{}
	for _, m := range strings.Split(ticket, ",") {
		v, _ := strconv.Atoi(m)
		myTicket = append(myTicket, v)
	}
	numberOfFields := len(myTicket)
	found := 0
	for found < numberOfFields-1 {
		for p, list := range matchPositions {
			missing := 0
			for _, l := range list {
				if _, ok := matches[l]; !ok {
					missing++
				}

			}
			if missing == 1 {
				for _, l := range list {

					if _, ok := matches[l]; !ok {
						matches[l] = p
						found++
					}
				}

			}
		}

	}

	factors := []int{}
	for field, p := range matches {
		if strings.HasPrefix(field, "departure") {
			factors = append(factors, p)
		}
	}

	mul := 1
	for _, f := range factors {
		mul *= myTicket[f]
	}

	fmt.Println(mul)
}

func readInput(path string) []string {
	file, err := ioutil.ReadFile(path)
	if err != nil {
		log.Fatal("could not open %s: %v", path, err)
	}
	data := strings.Split(string(file), "\n\n")
	return data
}
