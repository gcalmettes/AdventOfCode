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
	// ticket := input[1]
	nearby := input[2]

	// Authorized values for fields
	authorized := make(map[int]bool)

	for _, line := range strings.Split(rules, "\n") {
		parts := strings.Split(line, ": ")

		ranges := strings.Split(parts[1], " or ")
		for _, r := range ranges {
			bounds := strings.Split(r, "-")
			lower, _ := strconv.Atoi(bounds[0])
			upper, _ := strconv.Atoi(bounds[1])

			for i := lower; i <= upper; i++ {
				authorized[i] = true
			}
		}
	}

	wrong := []int{}

	for _, t := range strings.Split(nearby, "\n") {
		values := strings.Split(t, ",")
		for _, v := range values {
			n, _ := strconv.Atoi(v)
			if !authorized[n] {
				wrong = append(wrong, n)
			}
		}
	}

	total := 0
	for _, n := range wrong {
		total += n
	}
	fmt.Println(total)
}

func readInput(path string) []string {
	file, err := ioutil.ReadFile(path)
	if err != nil {
		log.Fatal("could not open %s: %v", path, err)
	}
	data := strings.Split(string(file), "\n\n")
	return data
}
