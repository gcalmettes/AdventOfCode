package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	rules, messages := readInput("input.txt")

	expressions := make(map[int]expression)

	numA, numB := 0, 0
	for _, rule := range rules {
		bits := strings.Split(rule, ": ")
		num, _ := strconv.Atoi(bits[0])
		r := bits[1]
		if r == `"b"` {
			expressions[num] = expression{"b", [][]int{}}
			numB = num
			continue
		}

		if r == `"a"` {
			expressions[num] = expression{"a", [][]int{}}
			numA = num
			continue
		}

		subRules := strings.Split(r, "|")

		subRuleSlice := make([][]int, len(subRules))
		for r, rule := range subRules {
			ruleBits := strings.Split(strings.TrimSpace(rule), " ")

			ruleSlice := make([]int, len(ruleBits))
			for i := range ruleBits {
				ruleSlice[i], _ = strconv.Atoi(ruleBits[i])
			}

			subRuleSlice[r] = ruleSlice
		}

		expressions[num] = expression{"", subRuleSlice}

	}

	re := regexp.MustCompile("^" + expressions[0].getText(numA, numB, expressions) + "$")
	valid := 0

	for _, t := range messages {
		if re.MatchString(t) {
			valid++
		}
	}

	fmt.Println(valid)
}

type expression struct {
	char     string
	subRules [][]int
}

func (e expression) getText(numA, numB int, exps map[int]expression) string {
	val := ""

	for _, sr := range e.subRules {

		if val != "" {
			val += "|"
		}

		for _, r := range sr {
			if r == numA || r == numB {
				val += exps[r].char
				continue
			}
			val += "(" + exps[r].getText(numA, numB, exps) + ")"
		}
	}

	return val
}

func readInput(path string) ([]string, []string) {
	file, err := ioutil.ReadFile(path)
	if err != nil {
		log.Fatal("could not open %s: %v", path, err)
	}
	data := strings.Split(string(file), "\n\n")

	rules := strings.Split(data[0], "\n")
	messages := strings.Split(data[1], "\n")

	return rules, messages
}
