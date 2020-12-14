package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

func main() {
	n, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	m := make(map[int]int)
	for _, id := range n.ids {
		m[id] = n.time + (id - (n.time % id))

	}
	var dif, targetId int
	for id, value := range m {
		d := value - n.time
		if dif == 0 {
			dif = d
			targetId = id
		}
		if d < dif {
			dif = d
			targetId = id
		}

	}
	fmt.Println(targetId * dif)
}

type notes struct {
	time int
	ids  []int
}

func readInput(path string) (notes, error) {
	f, err := ioutil.ReadFile(path)
	if err != nil {
		return notes{}, fmt.Errorf("could not open %s: %v", path, err)
	}

	lines := strings.Split(string(f), "\n")
	l := lines[1]
	var ids []int
	for _, s := range strings.Split(l, ",") {
		if s != "x" {
			i, _ := strconv.Atoi(s)
			ids = append(ids, i)
		}
	}
	j, _ := strconv.Atoi(lines[0])
	n := notes{j, ids}
	return n, nil
}
