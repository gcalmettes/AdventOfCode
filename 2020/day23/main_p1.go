package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	cups := []int{7, 9, 2, 8, 4, 5, 1, 3, 6}

	nRounds := 100
	i := 0
	for i < nRounds {
		// current cup will always be at position zero
		currentCup := cups[0]
		nextThree := cups[1:4]

		// remove the nextThree
		tmp := []int{}
		tmp = append(tmp, cups[0])
		tmp = append(tmp, cups[4:]...)

		// get min and max of left over cups
		min, max := minMax(tmp)

		destinationCupValue := currentCup
		var destinationCupIndex int

		for {
			destinationCupValue = destinationCupValue - 1

			if destinationCupValue < min {
				index, _ := find(tmp, max)
				destinationCupIndex = index + 1
				break
			} else {
				index, found := find(tmp, destinationCupValue)
				if found {
					destinationCupIndex = index + 1
					break
				}

			}
		}

		// insert the nextThree at correct position
		part1 := tmp[1:destinationCupIndex]
		part2 := nextThree
		var part3 []int
		part4 := tmp[:1]
		if destinationCupIndex >= len(tmp) {
			part3 = []int{}
		} else {
			part3 = tmp[destinationCupIndex:]
		}
		cups = make([]int, 0)
		for _, s := range [][]int{part1, part2, part3, part4} {
			cups = append(cups, s...)
		}
		i++
	}

	// find cup labeled as one
	index, _ := find(cups, 1)
	finalOrder := []int{}
	finalOrder = append(finalOrder, cups[index+1:]...)
	finalOrder = append(finalOrder, cups[:index]...)

	var final strings.Builder
	for _, v := range finalOrder {
		toAdd := strconv.Itoa(v)
		final.WriteString(toAdd)
	}
	fmt.Println(final.String())
}

func find(slice []int, value int) (int, bool) {
	for i, item := range slice {
		if item == value {
			return i, true
		}
	}
	return -1, false
}

func minMax(slice []int) (int, int) {
	var max int = slice[0]
	var min int = slice[0]
	for _, value := range slice {
		if max < value {
			max = value
		}
		if min > value {
			min = value
		}
	}
	return min, max
}
