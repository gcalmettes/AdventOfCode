package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func ParseLine(s string) []string {
	re := regexp.MustCompile("(.+) would (lose|gain) ([0-9]+) happiness units by sitting next to (.+).")
	return re.FindStringSubmatch(s)[1:]
}

// Perm calls f with each permutation of a.
func Perm(a []string, f func([]string)) {
	perm(a, f, 0)
}

// Permute the values at index i to len(a)-1.
func perm(a []string, f func([]string), i int) {
	if i > len(a) {
		f(a)
		return
	}
	perm(a, f, i+1)
	for j := i + 1; j < len(a); j++ {
		a[i], a[j] = a[j], a[i]
		perm(a, f, i+1)
		a[i], a[j] = a[j], a[i]
	}
}

func GetHappinessScore(guests []string, happiness map[string]map[string]int) int {
	maxHappy := 0
	Perm(guests, func(permGuests []string) {
		score := 0
		for i := range len(permGuests) {
			var p, n int
			if i == 0 {
				p = len(permGuests) - 1
				n = i + 1
			} else if i == len(permGuests)-1 {
				p = i - 1
				n = 0
			} else {
				p = i - 1
				n = i + 1
			}

			g := permGuests[i]
			g1 := permGuests[p]
			g2 := permGuests[n]
			if g != "me" {
				if g1 != "me" {
					score += happiness[g][g1]
					score += happiness[g1][g]
				}
				if g2 != "me" {
					score += happiness[g][g2]
					score += happiness[g2][g]

				}

			}
		}

		if score > maxHappy {
			maxHappy = score
		}

	})
	return maxHappy / 2
}

func main() {
	input, err := os.ReadFile("./inputs/13.txt")
	if err != nil {
		panic(err)
	}

	happiness := make(map[string]map[string]int)

	for _, line := range strings.Split(string(input), "\n") {
		if len(line) == 0 {
			continue
		}
		parsed := ParseLine(line)
		points, _ := strconv.Atoi(parsed[2])
		if parsed[1] == "lose" {
			points = -points
		}
		v, ok := happiness[parsed[0]]
		if ok {
			v[parsed[3]] = points
		} else {
			v = map[string]int{parsed[3]: points}
		}
		happiness[parsed[0]] = v
	}

	guests := []string{}
	for k, _ := range happiness {
		guests = append(guests, k)
	}

	p1 := GetHappinessScore(guests, happiness)

	guests = append(guests, "me")
	p2 := GetHappinessScore(guests, happiness)

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
