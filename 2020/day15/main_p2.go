package main

import (
	"fmt"
	"math"
)

func main() {
	n := []int{0, 14, 1, 3, 7, 9}

	spoken := make(map[int][]int)

	for i, el := range n {
		spoken[el] = []int{i + 1}
	}

	lastSpoken := n[len(n)-1]
	for i := len(n); i < 30000000; i++ {
		curr, total := spoken[lastSpoken], 0

		if len(curr) <= 1 {
			spoken[0] = append(spoken[0], i+1)
		} else {
			total = int(math.Abs(float64(curr[len(curr)-1]) - float64(curr[len(curr)-2])))
			spoken[total] = append(spoken[total], i+1)
		}

		lastSpoken = total
	}

	fmt.Println(lastSpoken)

}
