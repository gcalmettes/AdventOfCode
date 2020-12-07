package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	pToC, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	c := count(pToC, "shiny gold")
	fmt.Println(c)

}

func count(hash map[string]Bag, color string) int {
	total := 0

	toCheck := []Bag{hash[color]}

	for len(toCheck) > 0 {

		current := toCheck[0]
		toCheck = toCheck[1:]
		for c, n := range current.content {
			total += n
			for i := 0; i < n; i++ {
				toCheck = append(toCheck, hash[c])
			}
		}
	}

	return total
}

type Bag struct {
	color   string
	content map[string]int
}

func readInput(path string) (map[string]Bag, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("could not open %s: %v", path, err)
	}
	defer f.Close()

	d := make(map[string]Bag)

	s := bufio.NewScanner(f)
	for s.Scan() {
		t := s.Text()
		parts := strings.Split(t, " contain ")

		// outer bag
		color := strings.Split(parts[0], " bags")[0]

		// content
		content := make(map[string]int)
		if parts[1] != "no other bags." {

			children := strings.Split(parts[1], ", ")

			for _, c := range children {
				cparts := strings.Split(c, " ")
				innerBag := cparts[1] + " " + cparts[2]
				n, _ := strconv.Atoi(cparts[0])
				content[innerBag] = n
			}
		}
		d[color] = Bag{color, content}
	}
	return d, s.Err()
}
