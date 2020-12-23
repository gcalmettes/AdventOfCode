package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

func main() {
	equations := readInput("input.txt")

	total := 0
	for _, eq := range equations {
		total += evaluateEquation(eq)
	}

	fmt.Println(total)
}

func getDepth(input string) int {
	currDepth, maxDepth := 0, 0

	for _, b := range input {
		if string(b) == "(" {
			currDepth++
			if currDepth > maxDepth {
				maxDepth = currDepth
			}
		}

		if string(b) == ")" {
			currDepth--
		}
	}
	return maxDepth
}

func getExpressionAtDepth(input string, depth int) (string, int, int) {
	currDepth := 0
	track := false
	expression := ""
	start, end := 0, 0

	for i, b := range input {
		if string(b) == "(" {
			currDepth++
			if currDepth == depth {
				track = true
				start = i + 1
				continue
			}
		}
		// always getting the deepest.
		if string(b) == ")" {
			currDepth--
			if track {
				end = i - 1
				break
			}
		}

		if track {
			expression += string(b)
		}
	}
	return expression, start, end
}

func evaluateExpression(input string) int {

	bits := strings.Split(input, " ")

	acc := 0
	op := ""
	for _, bit := range bits {
		if bit == "+" || bit == "*" {
			op = bit
			continue
		}

		newNum, _ := strconv.Atoi(bit)

		switch op {
		case "":
			acc = newNum
		case "+":
			acc += newNum
		case "*":
			acc *= newNum
		}
	}

	return acc
}

func evaluateEquation(input string) int {
	for depth := getDepth(input); depth != 0; depth = getDepth(input) {
		exp, start, end := getExpressionAtDepth(input, depth)
		replacement := evaluateExpression(exp)
		input = input[:start-1] + strconv.Itoa(replacement) + input[end+2:]
	}

	return evaluateExpression(input)
}

func readInput(path string) []string {
	file, err := ioutil.ReadFile(path)
	if err != nil {
		log.Fatal("could not open %s: %v", path, err)
	}
	data := strings.Split(string(file), "\n")
	return data
}
