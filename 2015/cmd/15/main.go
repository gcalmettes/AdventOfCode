package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Ingredient struct {
	Name       string
	Capacity   int
	Durability int
	Flavor     int
	Texture    int
	Calories   int
}

type Recipe struct {
	Ingredients [4]Ingredient
	Quantities  [4]int
}

func (r *Recipe) GetScore(targetCalories int) int {
	cap := 0
	dur := 0
	flv := 0
	txt := 0
	cal := 0
	for i := range len(r.Ingredients) {
		if r.Quantities[i] == 0 {
			continue
		}
		cap += r.Quantities[i] * r.Ingredients[i].Capacity
		dur += r.Quantities[i] * r.Ingredients[i].Durability
		flv += r.Quantities[i] * r.Ingredients[i].Flavor
		txt += r.Quantities[i] * r.Ingredients[i].Texture
		cal += r.Quantities[i] * r.Ingredients[i].Calories
	}

	if cap < 0 {
		cap = 0
	}
	if dur < 0 {
		dur = 0
	}
	if flv < 0 {
		flv = 0
	}
	if txt < 0 {
		txt = 0
	}

	score := cap * dur * flv * txt
	if targetCalories > 0 {
		if cal != targetCalories {
			score = 0
		}

	}
	return score
}

func ParseLine(s string) []string {
	re := regexp.MustCompile("(.+): capacity (-?\\d+), durability (-?\\d+), flavor (-?\\d+), texture (-?\\d+), calories (-?\\d+)")
	return re.FindStringSubmatch(s)[1:]
}

func ingredientQuantities(yield func([4]int) bool) {
	max := 100
	for a := range max + 1 {
		for b := range max + 1 - a {
			for c := range max + 1 - a - b {
				d := max - a - b - c
				if a != 0 && b != 0 && c != 0 && d != 0 {
					yield([4]int{a, b, c, d})

				}
			}
		}
	}
}

func main() {
	input, err := os.ReadFile("./inputs/15.txt")
	if err != nil {
		panic(err)
	}

	ingredients := [4]Ingredient{Ingredient{}, Ingredient{}, Ingredient{}, Ingredient{}}

	i := 0
	for _, line := range strings.Split(string(input), "\n") {
		if len(line) == 0 {
			continue
		}
		parsed := ParseLine(line)
		cap, _ := strconv.Atoi(parsed[1])
		dur, _ := strconv.Atoi(parsed[2])
		flv, _ := strconv.Atoi(parsed[3])
		txt, _ := strconv.Atoi(parsed[4])
		cal, _ := strconv.Atoi(parsed[5])
		ingredients[i] = Ingredient{parsed[0], cap, dur, flv, txt, cal}
		i += 1
	}

	var p1, p2 int

	for quant := range ingredientQuantities {
		rec := Recipe{ingredients, quant}
		score1 := rec.GetScore(-1)
		score2 := rec.GetScore(500)
		if score1 > p1 {
			p1 = score1
		}
		if score2 > p2 {
			p2 = score2
		}
	}

	fmt.Printf("part1: %d\n", p1)
	fmt.Printf("part2: %d\n", p2)
}
