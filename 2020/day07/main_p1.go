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
	// parent to child
	pToC, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	// child to parent
	cToP := make(map[string][]string)
	for _, parent := range pToC {
		for child, _ := range parent.content {
			if _, exists := cToP[child]; !exists {
				cToP[child] = []string{}
			}
			cToP[child] = append(cToP[child], parent.color)

		}
	}

	parents := crawl(cToP, []string{"shiny gold"})
	fmt.Println(len(parents))
}

func crawl(hash map[string][]string, toCheck []string) map[string]bool {
	parents := make(map[string]bool)
	for len(toCheck) > 0 {

		current := toCheck[0]
		toCheck = toCheck[1:]
		for _, p := range hash[current] {
			parents[p] = true
			toCheck = append(toCheck, p)

		}
	}
	return parents
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
