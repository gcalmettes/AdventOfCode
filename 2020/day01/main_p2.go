package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	expenses, err := readInput("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	for i, e1 := range expenses[:len(expenses)-2] {
		for j, e2 := range expenses[i : len(expenses)-1] {
			for _, e3 := range expenses[j:] {
				total := e1 + e2 + e3
				if total == 2020 {
					fmt.Println(e1, e2, e3, e1*e2*e3)
				}
			}

		}

	}
}

func readInput(path string) ([]int, error) {

	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("could not open %s: %v", path, err)
	}
	defer f.Close()
	var data []int

	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		var d int
		_, err := fmt.Sscanf(scanner.Text(), "%d", &d)
		if err != nil {
			log.Fatal(err)
		}
		data = append(data, d)
	}
	return data, scanner.Err()
}
