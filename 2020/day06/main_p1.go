package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	groups, err := readInput("input.txt")
	if err != nil {
		fmt.Println(err)
	}
	total := 0
	for i := 0; i < len(groups); i++ {
		g := &groups[i]
		*g = strings.ReplaceAll(*g, "\n", "")

		counter := make(map[string]int)
		for _, c := range []byte(*g) {
			counter[string(c)]++
		}
		total += len(counter)
	}
	fmt.Println(total)
}

func readInput(path string) ([]string, error) {
	f, err := ioutil.ReadFile(path)
	if err != nil {
		return []string{}, fmt.Errorf("could not open %s: %v", path, err)
	}

	data := strings.Split(strings.TrimSpace(string(f)), "\n\n")

	return data, nil
}
