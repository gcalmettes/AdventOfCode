package main

import (
	"crypto/md5"
	"fmt"
	"strings"
)

const (
	INPUT string = "yzbqklnj"
)

func main() {
	i := 0

	p1 := false

	for true {
		candidate := fmt.Sprintf("%s%d", INPUT, i)
		hash := fmt.Sprintf("%x", md5.Sum([]byte(candidate)))

		if !p1 && strings.HasPrefix(hash, "00000") {
			fmt.Printf("part1: %d\n", i)
			p1 = true
		}
		if strings.HasPrefix(hash, "000000") {
			fmt.Printf("part2: %d\n", i)
			break
		}
		i += 1
	}
}
