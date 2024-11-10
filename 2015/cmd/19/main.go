package main

import (
	"fmt"
	"os"
	"strings"
)

func GetValidMolecules(replacements *[][2]string, molecule string) map[string]bool {
	valid := map[string]bool{}
	for _, rpl := range *replacements {
		size := len(rpl[0])

		for i := range len(molecule) + 1 - size {
			pattern := molecule[i : i+size]
			if pattern == rpl[0] {
				var new string
				if i == 0 {
					new = rpl[1]
				} else {
					new = molecule[:i] + rpl[1]
				}
				new += molecule[i+size:]
				valid[new] = true
			}
		}
	}
	return valid
}

func main() {
	input, err := os.ReadFile("./inputs/19.txt")
	if err != nil {
		panic(err)
	}

	parts := strings.SplitN(string(input), "\n\n", 2)

	molecule := strings.TrimSpace(parts[1])
	replacements := [][2]string{}

	for _, line := range strings.Split(parts[0], "\n") {
		if len(line) == 0 {
			continue
		}
		p := strings.SplitN(line, " => ", 2)
		replacements = append(replacements, [2]string{p[0], p[1]})
	}

	p1 := GetValidMolecules(&replacements, molecule)

	invReplacements := map[string]string{}
	groups := map[int][]string{}
	max := 0
	for _, r := range replacements {
		invReplacements[r[1]] = r[0]
		l := len(r[1])
		groups[l] = append(groups[l], r[1])
		if l > max {
			max = l
		}
	}

	p2 := 0
out:
	for {
		for i := range max {
			size := max - i
		next:
			for {
				before := molecule
				for _, p := range groups[size] {
					for {
						after := strings.Replace(molecule, p, invReplacements[p], 1)
						if after != molecule {
							p2 += 1
						} else {
							break
						}
						molecule = after
						if molecule == "e" {
							break out
						}

					}
				}
				if molecule == before {
					break next
				}
			}
		}
	}
	fmt.Printf("part1: %d\n", len(p1))
	fmt.Printf("part2: %d\n", p2)
}
