package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

func main() {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		log.Panic("could not read file: %v", err)
	}
	input := strings.Split(strings.TrimRight(string(data), "\n"), "\n")
	al := parse(input)
	fal := buildFullAllergenList(al)

	res := 0
	for _, l := range input {
		parts := strings.Split(l, " (")
		ingredients := strings.Split(parts[0], " ")
		for _, i := range ingredients {
			if _, found := fal[i]; !found {
				res++
			}
		}
	}
	fmt.Println(res)
}

func solve(fal Set, input []string) int {
	res := 0
	for _, l := range input {
		parts := strings.Split(l, " (")
		ingredients := strings.Split(parts[0], " ")
		for _, i := range ingredients {
			if _, found := fal[i]; !found {
				res++
			}
		}
	}
	return res
}

type Set map[string]struct{}

func intersect(a, b Set) Set {
	res := make(Set)
	for k := range a {
		if _, found := b[k]; found {
			res[k] = struct{}{}
		}
	}
	return res
}

func union(a, b Set) Set {
	res := make(Set)
	for k := range a {
		res[k] = struct{}{}
	}
	for k := range b {
		res[k] = struct{}{}
	}
	return res
}

func arrayToSet(a []string) Set {
	res := make(Set)
	for _, e := range a {
		res[e] = struct{}{}
	}
	return res
}

func parse(input []string) map[string]Set {
	allergenList := make(map[string]Set)
	for _, l := range input {
		parts := strings.Split(l, " (")
		ingredients := strings.Split(parts[0], " ")
		// Remove "contain" and closing parens
		allergens := strings.Split(parts[1][9:len(parts[1])-1], ", ")
		for _, a := range allergens {
			if _, exists := allergenList[a]; exists {
				allergenList[a] = intersect(allergenList[a], arrayToSet(ingredients))
			} else {
				allergenList[a] = arrayToSet(ingredients)
			}
		}
	}
	return allergenList
}

func buildFullAllergenList(al map[string]Set) Set {
	res := make(Set)
	for _, v := range al {
		res = union(res, v)
	}
	return res
}
