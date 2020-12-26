package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"sort"
	"strings"
)

func main() {
	data, err := ioutil.ReadFile("input.txt")
	if err != nil {
		log.Panic("could not read file: %v", err)
	}
	input := strings.Split(strings.TrimRight(string(data), "\n"), "\n")
	al := parse(input)
	reduce(al)
	// fmt.Println(solve(al))

	mapped := make([][]string, 0, len(al))
	for k, v := range al {
		mapped = append(mapped, []string{k, firstElem(v)})
	}
	sort.Slice(mapped, func(i, j int) bool { return mapped[i][0] < mapped[j][0] })
	res := make([]string, len(mapped))
	for i, e := range mapped {
		res[i] = e[1]
	}

	fmt.Println(strings.Join(res, ","))
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

func firstElem(a Set) string {
	for k := range a {
		return k
	}
	panic("No element in set")
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

func reduce(al map[string]Set) {
	for reduced := 1; reduced > 0; {
		reduced = 0
		for k, v := range al {
			if len(v) == 1 {
				toDelete := firstElem(v)
				for ok, ov := range al {
					if _, found := ov[toDelete]; found && k != ok {
						reduced++
						delete(ov, toDelete)
					}
				}
			}
		}
	}
}
