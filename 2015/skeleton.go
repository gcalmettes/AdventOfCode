package main

import (
	"fmt"
	"os"
)

func main() {
	input, err := os.ReadFile("./inputs/01.txt")
	if err != nil {
		panic(err)
	}
	fmt.Printf("data: %v\n", input)
}
