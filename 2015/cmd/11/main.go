package main

import (
	"fmt"
	"regexp"
	"strings"
)

func validateStraight(s string) bool {
	if len(s) < 3 {
		return false
	}
	for i := 1; i < len(s)-1; i++ {
		if s[i]-s[i-1] == 1 && s[i+1]-s[i] == 1 {
			return true
		}
	}
	return false
}

func validateForbiddenLetters(s string) bool {
	for _, letter := range []string{"i", "o", "l"} {
		if strings.Contains(s, letter) {
			return false
		}
	}
	return true
}

func validateRepeatedLetters(s string) bool {
	var regex strings.Builder
	for letter := 'a'; letter <= 'z'; letter++ {
		regex.WriteString(string(letter))
		regex.WriteString("{2}|")
	}
	re := regexp.MustCompile(strings.TrimSuffix(regex.String(), "|"))
	matches := re.FindAllStringSubmatch(s, -1)

	set := map[string]bool{}
	for _, m := range matches {
		set[m[0]] = true
	}
	return len(matches) > 1 && len(matches) == len(set)
}

func validatePassword(s string) bool {
	return validateStraight(s) && validateForbiddenLetters(s) && validateRepeatedLetters(s)
}

func GetNextPassword(s string) string {
	runes := []rune{}
	for _, r := range s {
		runes = append(runes, r)
	}

	idx := len(s) - 1
	toIncrement := runes[idx]
	if toIncrement < 'z' {
		// simple case, increment
		runes[idx] = runes[idx] + 1
	} else {
		// wrap around
		runes[idx] = 'a'
		for {
			idx = idx - 1
			if runes[idx] < 'z' {
				runes[idx] = runes[idx] + 1
				break
			} else {
				runes[idx] = 'a'
				if idx == 0 {
					runes = append([]rune{'a'}, runes...)
					idx = 1
					break
				}
			}
		}
	}
	return string(runes)
}

func GetNextValidPassword(s string) string {
	next := GetNextPassword(s)
	for {
		if validatePassword(next) {
			break
		}
		next = GetNextPassword(next)
	}
	return next
}

const INPUT string = "vzbxkghb"

func main() {

	p1 := GetNextValidPassword(INPUT)
	p2 := GetNextValidPassword(p1)

	fmt.Printf("part1: %s\n", p1)
	fmt.Printf("part2: %s\n", p2)
}
